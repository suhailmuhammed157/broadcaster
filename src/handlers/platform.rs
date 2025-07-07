use actix_web::{Error, HttpResponse, post, web};
use serde_json::json;

use crate::AppState;

#[post("/register")]
pub async fn register(
    app_state: web::Data<AppState>,
    // register_json: web::Json<RegisterModel>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", app_state.platforms);

    Ok(HttpResponse::Ok().json(json!({
        "Status":200,
        "Message": format!("Platform added")
    })))
}
