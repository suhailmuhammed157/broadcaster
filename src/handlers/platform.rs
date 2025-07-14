use actix_web::{Error, HttpResponse, post, web};
use jwt_lib::Platform;
use serde_json::json;

use crate::{AppState, middlewares::platform_auth::AuthPlatform, models::platform::AddPlatform};

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

    let added_platform = Platform {
        platform_name: register_json.platform_name.clone(),
        platform_id: id,
    };

    let token = jwt_lib::get_jwt(added_platform);

    match token {
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "Status":200,
            "Message": format!("unable to generate token"),
            "Error":e.to_string()
        }))),
        Ok(val) => Ok(HttpResponse::Ok().json(json!({
            "Status":200,
            "Message": format!("Platform added with id {id}"),
            "Data":{
                "platform_id":id,
                "token":val
            }
        }))),
    }
}

#[post("/broadcast")]
pub async fn broadcast(
    app_state: web::Data<AppState>,
    _: AuthPlatform,
) -> Result<HttpResponse, Error> {
    let clients_guard = app_state.clients.lock().unwrap(); // lock the mutex
    for (id, tx) in clients_guard.iter() {
        match tx.send(String::from("hey")) {
            Ok(_) => {}
            Err(e) => {
                println!("Error or boradcasting to client id {id} with error {e}");
                continue;
            }
        }
    }
    Ok(HttpResponse::Ok().json(json!({
        "Status":200,
        "Message": format!("Message broadcasted successfully"),
    })))
}
