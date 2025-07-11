use std::{collections::HashMap, sync::Mutex};

use actix_web::{App, HttpServer, web};
use tokio::sync::mpsc;
mod handlers;
mod models;
mod routes;

pub struct AppState {
    pub platforms: Mutex<HashMap<String, i64>>,
    pub clients: Mutex<HashMap<String, mpsc::UnboundedSender<String>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Server is Listining on 127.0.0.1:9090");

    let app_state = web::Data::new(AppState {
        platforms: Mutex::new(HashMap::new()),
        clients: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || App::new().app_data(app_state.clone()).configure(routes))
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}

fn routes(app: &mut web::ServiceConfig) {
    app.configure(routes::platform::config)
        .configure(routes::ws::config);
}
