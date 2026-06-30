use std::collections::HashMap;

use actix_htmx::Htmx;
use actix_identity::Identity;
use actix_web::{
    get, post,
    web::{self, Redirect},
    HttpMessage, HttpRequest, HttpResponse, Responder, ResponseError,
};
use anyhow::Context;
use secrecy::{SecretBox, SecretString};
use serde::Deserialize;
use thiserror::Error;
use ui::{
    index::{IndexRoute, UiComponent},
    user::sign_up::SignUp,
};

use crate::{
    domain::{
        artist::ports::ArtistService,
        event::ports::EventService,
        user::{
            models::user::{CreateUserRequest, UserEmail, UserName, UserPassword},
            ports::UserService,
        },
    },
    inbound::http::{handlers::index_markup, AppState},
};

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
async fn signup_form(htmx: Htmx) -> impl Responder {
    let body = if htmx.is_htmx {
        SignUp {}.render_html()
    } else {
        index_markup("Sign Up", IndexRoute::SignUp, None).render_html()
    };

    HttpResponse::Ok().content_type("text/html").body(body)
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

    let username = UserName::try_new(form.username.clone()).context("Invalid username")?;
    let email = UserEmail::try_new(form.email.clone()).context("Invalid Email")?;
    let password = UserPassword::new(SecretString::from(form.password.clone()));
    let req = CreateUserRequest::new(username, email, password);

    let user = app_state
        .user_service
        .create_user(&req)
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
