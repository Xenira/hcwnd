use actix_utils::future::Ready;
use actix_web::{
    dev::{Payload, ServiceRequest},
    FromRequest,
};

use crate::{
    model::{Act, Stage},
    prelude::*,
};

pub struct Event {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub stages: Vec<Stage>,
    pub acts: Vec<Act>,
}

impl FromRequest for Event {
    type Error = crate::error::Error;
    type Future = Ready<Result<Self>>;

    fn from_request(req: &ServiceRequest, payload: &mut Payload) -> Self::Future {
        // let event_id = req.
        todo!()
    }
}
