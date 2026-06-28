use actix_htmx::Htmx;
use actix_identity::Identity;
use actix_web::{web, HttpMessage as _, HttpRequest, HttpResponse, Responder, ResponseError};
use anyhow::Context as _;
use secrecy::SecretString;
use serde::Deserialize;
use thiserror::Error;
use ui::{
    index::{IndexRoute, UiComponent},
    user::{sign_in::SignIn, sign_up::SignUp},
};

use crate::{
    domain::{
        artist::ports::ArtistService,
        event::ports::EventService,
        user::{
            models::user::{AuthenticateUserRequest, UserEmail, UserPassword},
            ports::UserService,
        },
    },
    inbound::http::{handlers::index_markup, AppState},
};

pub fn configure<ES, AS, US>(cfg: &mut web::ServiceConfig)
where
    ES: EventService + 'static,
    AS: ArtistService + 'static,
    US: UserService + 'static,
{
    cfg.route("", web::get().to(login_form))
        .route("", web::post().to(login::<ES, AS, US>));
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

async fn login_form(htmx: Htmx) -> impl Responder {
    let body = if htmx.is_htmx {
        SignIn {}.render_html()
    } else {
        index_markup("Sign Up", IndexRoute::Login, None).render_html()
    };

    HttpResponse::Ok().content_type("text/html").body(body)
}

#[derive(Deserialize)]
struct LoginFormData {
    email: String,
    password: String,
}

async fn login<ES: EventService, AS: ArtistService, US: UserService>(
    app_state: web::Data<AppState<ES, AS, US>>,
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
