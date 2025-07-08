use std::{collections::HashMap, sync::Mutex};

use actix_web::{App, HttpServer, web};
mod handlers;
mod models;
mod routes;

pub struct AppState {
    pub platforms: Mutex<HashMap<String, i64>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Server is Listining on 127.0.0.1:9090");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                platforms: Mutex::new(HashMap::new()),
            }))
            .configure(routes)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig) {
    app.configure(routes::platform::config);
}
