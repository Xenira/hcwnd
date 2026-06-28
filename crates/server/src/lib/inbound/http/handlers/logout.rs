use actix_htmx::Htmx;
use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(logout));
}

async fn logout(identity: Option<Identity>, htmx: Htmx) -> impl Responder {
    if let Some(identity) = identity {
        identity.logout();
    }

    if htmx.is_htmx {
        htmx.redirect("/");
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::TemporaryRedirect()
            .append_header(("Location", "/"))
            .finish()
    }
}
