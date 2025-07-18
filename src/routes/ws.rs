use actix_web;
use actix_web::web;

use crate::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(web::scope("/ws").service(handlers::ws::connect));
}
