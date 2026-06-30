use actix_htmx::Htmx;
use actix_identity::Identity;
use actix_web::{
    HttpMessage as _, HttpRequest, HttpResponse, Responder, ResponseError, get, post, web,
};
use anyhow::Context as _;
use secrecy::SecretString;
use serde::Deserialize;
use thiserror::Error;

use crate::{
    domain::user::{
        models::user::{AuthenticateUserRequest, UserEmail, UserPassword},
        ports::UserService,
    },
    inbound::http::{AppState, user::Locale},
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(login_form).service(login);
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
async fn login_form(locale: Locale<'_>, htmx: Htmx) -> impl Responder {
    let state = api::UiState::from(&locale);

    let body = if htmx.is_htmx {
        ui::user::sign_in::render(&state.locale)
    } else {
        ui::user::sign_in::full_page(&state)
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}

#[derive(Deserialize)]
struct LoginFormData {
    email: String,
    password: String,
}

#[post("")]
async fn login(
    app_state: web::Data<AppState>,
    request: HttpRequest,
    form: web::Form<LoginFormData>,
    htmx: Htmx,
) -> Result<impl Responder, HandlerError> {
    let email = UserEmail::try_new(form.email.clone()).context("Invalid Email")?;
    let password = UserPassword::new(SecretString::from(form.password.clone()));
    let req = AuthenticateUserRequest::new(email, password);

    let user = app_state
        .user_service
        .authenticate_user(&req)
        .await
        .context("Failed to authenticate user")?;

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
