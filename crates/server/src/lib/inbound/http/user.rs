use std::ops::Deref;

use actix_identity::IdentityExt;
use actix_utils::future::Ready;
use actix_web::{dev::Payload, web, FromRequest, HttpMessage, HttpRequest};
use futures::future::LocalBoxFuture;
use log::{error, info};
use uuid::Uuid;

use crate::{
    domain::{
        artist::ports::ArtistService,
        event::ports::EventService,
        user::{
            models::user::{User, UserId},
            ports::UserService,
        },
    },
    inbound::http::AppState,
};

pub struct UserExtractor<ES, AS, US>
where
    ES: EventService + Clone + Sync + Send + 'static,
    AS: ArtistService + Clone + Sync + Send + 'static,
    US: UserService + Clone + Sync + Send + 'static,
{
    pub user: User,
    _event_service: std::marker::PhantomData<ES>,
    _artist_service: std::marker::PhantomData<AS>,
    _user_service: std::marker::PhantomData<US>,
}

impl<ES, AS, US> Deref for UserExtractor<ES, AS, US>
where
    ES: EventService + Clone + Sync + Send + 'static,
    AS: ArtistService + Clone + Sync + Send + 'static,
    US: UserService + Clone + Sync + Send + 'static,
{
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

impl<ES, AS, US> FromRequest for UserExtractor<ES, AS, US>
where
    ES: EventService + Clone + Sync + Send + 'static,
    AS: ArtistService + Clone + Sync + Send + 'static,
    US: UserService + Clone + Sync + Send + 'static,
{
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let identity = req.get_identity();
        let app_state = req.app_data::<web::Data<AppState<ES, AS, US>>>().cloned();

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
            Ok(UserExtractor {
                user: app_state
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
                    .ok_or_else(|| actix_web::error::ErrorUnauthorized("User not found"))?,
                _event_service: std::marker::PhantomData,
                _artist_service: std::marker::PhantomData,
                _user_service: std::marker::PhantomData,
            })
        })
    }
}
