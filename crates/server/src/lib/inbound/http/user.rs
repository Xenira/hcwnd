use std::borrow::Cow;

use actix_identity::IdentityExt;
use actix_utils::future::{Ready, ready};
use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use api::UiState;
use futures::future::LocalBoxFuture;
use log::{error, info};
use uuid::Uuid;

use crate::{
    domain::user::{
        models::user::{User, UserId},
        ports::UserService,
    },
    inbound::http::AppState,
};

impl FromRequest for User {
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let identity = req.get_identity();
        let app_state = req.app_data::<web::Data<AppState>>().cloned();

        Box::pin(async move {
            let identity = identity?.id()?;
            let user_id = UserId::new(Uuid::parse_str(&identity).map_err(|e| {
                dbg!("Failed to parse user ID from identity: {}", &e);
                actix_web::error::ErrorBadRequest(format!("Invalid user ID format: {}", e))
            })?);
            let app_state = app_state.ok_or_else(|| {
                error!("App state is not configured");
                actix_web::error::ErrorInternalServerError("App state is not configured")
            })?;

            info!("Extracting user with ID: {:?}", user_id);
            app_state
                .user_service
                .get_user(&user_id)
                .await
                .map_err(|e| {
                    error!("Failed to retrieve user from service: {}", &e);
                    actix_web::error::ErrorInternalServerError(format!(
                        "Failed to retrieve user: {}",
                        e
                    ))
                })?
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("User not found"))
        })
    }
}

pub struct Locale<'a>(Cow<'a, str>);

impl From<Locale<'_>> for UiState {
    fn from(locale: Locale) -> Self {
        UiState {
            user: None,
            locale: locale.0.into_owned(),
        }
    }
}

impl From<&Locale<'_>> for UiState {
    fn from(locale: &Locale) -> Self {
        UiState {
            user: None,
            locale: locale.0.clone().into_owned(),
        }
    }
}

impl FromRequest for Locale<'_> {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let locale = req
            .headers()
            .get("Accept-Language")
            .and_then(|value| {
                value
                    .to_str()
                    .ok()
                    .map(|s| s.split(',').next().unwrap_or("en").to_string())
            })
            .unwrap_or_else(|| "en".to_string());

        ready(Ok(Locale(Cow::Owned(locale))))
    }
}
