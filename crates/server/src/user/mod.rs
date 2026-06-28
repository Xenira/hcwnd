use crate::prelude::*;

mod sign_in;
mod sign_up;
mod validator;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(sign_in::configure)
        .configure(sign_up::configure);
}
