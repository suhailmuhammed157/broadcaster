use std::collections::HashMap;

use actix_web::{Error, HttpResponse, post, web};
use serde_json::json;

use crate::{AppState, models::platform::AddPlatform};

#[post("/register")]
pub async fn register(
    app_state: web::Data<AppState>,
    register_json: web::Json<AddPlatform>,
) -> Result<HttpResponse, Error> {
    if register_json.platform_name == "" {
        return Ok(HttpResponse::BadRequest().json(json!({
            "Status":404,
            "Message": format!("platform_name is missing")

        })));
    }

    let mut platform = app_state.platforms.lock().unwrap();

    if platform.contains_key(&register_json.platform_name) {
        return Ok(HttpResponse::BadRequest().json(json!({
            "Status":404,
            "Message": format!("platform already exists")
        })));
    }

    let id: i64 = platform.len() as i64 + 1;
    platform.insert(register_json.platform_name.clone(), id);
    let mut response = HashMap::new();
    response.insert("platform_id", id);

    Ok(HttpResponse::Ok().json(json!({
        "Status":200,
        "Message": format!("Platform added with id {id}"),
        "Data":response
    })))
}
