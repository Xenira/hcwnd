use actix_web::{
    HttpResponse, Responder, get,
    web::{self, ServiceConfig},
};
use mime_guess::from_path;
use rust_embed::{Embed, RustEmbed};

#[derive(Embed)]
#[folder = "../../node_modules/@phosphor-icons/web/src/"]
#[include = "**/*.woff2"]
#[include = "**/*.woff"]
#[include = "**/*.ttf"]
#[include = "**/*.css"]
struct Icons;

#[derive(Embed)]
#[folder = "../../node_modules/@fontsource/open-sans/"]
#[include = "files/*.woff2"]
#[include = "files/*.woff"]
#[include = "index.css"]
struct OpenSans;

const MU_CSS: &str =
    include_str!("../../../../../../../node_modules/@digicreon/mucss/dist/mu.slate.css");
const STYLE_CSS: &str = include_str!(concat!(env!("ASSET_OUT_DIR"), "/style.css"));
const HTMX_JS: &str = include_str!("../../../../../../../node_modules/htmx.org/dist/htmx.min.js");

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(mu_css)
        .service(style_css)
        .service(htmx_js)
        .service(icons)
        .service(fonts);
}

#[get("/mu.css")]
async fn mu_css() -> impl Responder {
    HttpResponse::Ok().content_type("text/css").body(MU_CSS)
}

#[get("/style.css")]
async fn style_css() -> impl Responder {
    HttpResponse::Ok().content_type("text/css").body(STYLE_CSS)
}

#[get("/htmx.min.js")]
async fn htmx_js() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(HTMX_JS)
}

#[get("/icons/{_:.*}")]
async fn icons(path: web::Path<String>) -> impl Responder {
    handle_embedded_file::<Icons>(&path)
}

#[get("/fonts/{_:.*}")]
async fn fonts(path: web::Path<String>) -> impl Responder {
    handle_embedded_file::<OpenSans>(&path)
}

fn handle_embedded_file<T: RustEmbed>(path: &str) -> HttpResponse {
    match T::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}
