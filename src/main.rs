use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::{App, HttpServer, web};

use tokio::sync::mpsc;
mod handlers;
mod middlewares;
mod models;
mod routes;

pub struct Room {
    pub platform_id: i64,
    pub clients: Mutex<HashMap<String, mpsc::UnboundedSender<String>>>,
}

pub struct AppState {
    pub rooms: Mutex<HashMap<String, Arc<Room>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host_address = utils::HOST.clone();
    let port = utils::PORT.clone();

    log::info!("Server is Listining on {}:{}", host_address, port);

    let app_state = web::Data::new(AppState {
        rooms: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || App::new().app_data(app_state.clone()).configure(routes))
        .bind((host_address, port))?
        .run()
        .await
}

fn routes(app: &mut web::ServiceConfig) {
    app.configure(routes::platform::config)
        .configure(routes::ws::config);
}
