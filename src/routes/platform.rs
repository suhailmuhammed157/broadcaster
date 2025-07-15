use actix_web;
use actix_web::web;

use crate::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/platform")
            .service(handlers::platform::register)
            .service(handlers::platform::broadcast)
            .service(handlers::platform::renew_token),
    );
}
