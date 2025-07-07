use std::collections::HashMap;

use actix_web::{App, HttpServer, web};
mod handlers;
mod routes;

pub struct AppState {
    pub platforms: HashMap<String, i32>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Server is Listining on 127.0.0.1:9090");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                platforms: HashMap::new(),
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
