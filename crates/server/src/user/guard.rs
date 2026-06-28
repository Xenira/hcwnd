use std::future::{ready, Ready};

use actix_identity::IdentityExt;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use futures::future::LocalBoxFuture;
use log::debug;

#[derive(Debug, Clone, Copy)]
pub struct SignedInGuard {
    inverted: bool,
}

impl<S, B> Transform<S, ServiceRequest> for SignedInGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = SignedInGuardMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SignedInGuardMiddleware {
            service,
            inverted: self.inverted,
        }))
    }
}

pub struct SignedInGuardMiddleware<S> {
    service: S,
    inverted: bool,
}

impl<S, B> Service<ServiceRequest> for SignedInGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match req.get_identity() {
            Ok(identity) => {
                if self.inverted {
                    debug!("Redirecting signed-in user to home page");
                    let res = req.into_response(
                        actix_web::HttpResponse::Found()
                            .insert_header(("Location", "/"))
                            .finish(),
                    );
                    return Box::pin(async { Ok(res) });
                }
                debug!("User is signed in with ID: {}", identity.id);
            }
            Err(_) => {
                if !self.inverted {
                    debug!("Redirecting unsigned user to sign-in page");
                    let res = req.into_response(
                        actix_web::HttpResponse::Found()
                            .insert_header(("Location", "/sign-in"))
                            .finish(),
                    );
                    return Box::pin(async { Ok(res) });
                }
                debug!("User is not signed in");
            }
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            Ok(res)
        })
    }
}
