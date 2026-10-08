use actix_htmx::Htmx;
use actix_identity::Identity;
use actix_web::{
    get, post,
    web::{self},
    HttpMessage, HttpRequest, HttpResponse, Responder, ResponseError,
};
use anyhow::Context;
use domain::user::{
    commands::CreateUser,
    models::user::{CreateUserRequest, UserEmail, UserName, UserPassword},
    ports::UserService,
};
use secrecy::SecretString;
use serde::Deserialize;
use thiserror::Error;

use crate::inbound::http::{user::Locale, AppState};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(signup_form).service(signup);
}

#[derive(Error, Debug)]
enum HandlerError {
    #[error(transparent)]
    ServiceError(#[from] anyhow::Error),
}

impl ResponseError for HandlerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            HandlerError::ServiceError(e) => {
                HttpResponse::InternalServerError().body(format!("Service error: {e}"))
            }
        }
    }
}

#[get("")]
async fn signup_form(locale: Locale<'_>, htmx: Htmx) -> impl Responder {
    let state = api::UiState::from(&locale);

    let body = if htmx.is_htmx {
        ui::user::sign_up::render(&state.locale)
    } else {
        ui::user::sign_up::full_page(&state)
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}

#[derive(Deserialize)]
struct SignupFormData {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
    privacy_policy: bool,
}

#[post("")]
async fn signup(
    app_state: web::Data<AppState>,
    request: HttpRequest,
    form: web::Form<SignupFormData>,
    htmx: Htmx,
) -> Result<impl Responder, HandlerError> {
    if form.password != form.confirm_password {
        return Err(anyhow::anyhow!("Passwords do not match").into());
    }
    if !form.privacy_policy {
        return Err(anyhow::anyhow!("You must agree to the privacy policy").into());
    }

    let form: SignupFormData = form.into_inner();
    let req = CreateUser::new(
        form.username,
        form.email,
        SecretString::from(form.password.clone()),
    );

    let user = app_state
        .user_service
        .create_user(req)
        .await
        .context("Failed to create user")?;

    Identity::login(&request.extensions(), user.id().as_ref().to_string())
        .context("Failed to create session")?;

    if htmx.is_htmx {
        htmx.redirect("/");
        Ok(HttpResponse::Created().finish())
    } else {
        Ok(HttpResponse::TemporaryRedirect()
            .append_header(("Location", "/"))
            .finish())
    }
}
