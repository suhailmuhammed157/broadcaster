use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::{Error, HttpResponse, post, web};
use jwt_lib::Platform;
use serde_json::json;

use crate::{
    AppState, Room,
    middlewares::platform_auth::AuthPlatform,
    models::platform::{AddPlatform, Broadcast},
};

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

    // let mut platform = app_state.platforms.lock().unwrap();
    let mut rooms = app_state.rooms.lock().unwrap();

    if rooms.contains_key(&register_json.platform_name) {
        return Ok(HttpResponse::BadRequest().json(json!({
            "Status":404,
            "Message": format!("platform already exists")
        })));
    }

    let id: i64 = rooms.len() as i64 + 1;

    let new_room = Arc::new(Room {
        platform_id: id,
        clients: Mutex::new(HashMap::new()),
    });

    rooms.insert(register_json.platform_name.clone(), new_room);

    let platform = Platform {
        platform_id: id,
        platform_name: register_json.platform_name.clone(),
    };

    let token = jwt_lib::get_jwt(platform);

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
    platform_details: AuthPlatform,
    broadcast_json: web::Json<Broadcast>,
) -> Result<HttpResponse, Error> {
    let rooms = app_state.rooms.lock().unwrap();

    if !rooms.contains_key(&platform_details.0.platform_name) {
        return Ok(HttpResponse::Unauthorized().json(json!({
            "Status":401,
            "Message": format!("platform not found"),
        })));
    }

    if broadcast_json.message == "" {
        return Ok(HttpResponse::BadRequest().json(json!({
            "Status":400,
            "Message": format!("message is required"),
        })));
    }

    let room = rooms.get(&platform_details.0.platform_name).unwrap();

    let clients = room.clients.lock().unwrap();

    for (id, tx) in clients.iter() {
        match tx.send(broadcast_json.message.clone()) {
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
