use std::time::Duration;

use actix_web::{Error, HttpRequest, HttpResponse, get, rt, web};
use actix_ws::{Message, handle};
use serde_json::json;
use tokio::sync::mpsc;

use crate::{AppState, models::ws::ConnectQuery};

fn remove_client(app_state: &AppState, platform_name: String, client_id_to_remove: String) {
    let mut rooms = app_state.rooms.lock().unwrap();
    if let Some(room) = rooms.iter_mut().find(|r| r.platform_name == platform_name) {
        let mut clients = room.clients.lock().unwrap();

        let removed_item = clients.remove(&client_id_to_remove);

        // free up the space of removed items
        clients.shrink_to_fit();

        if let Some(item) = removed_item {
            println!("client {:?} has been removed", item);
        } else {
            println!("Item has been removed before");
        }
    } else {
        println!("room not found");
    }
}

#[get("")]
pub async fn connect(
    app_state: web::Data<AppState>,
    query: web::Query<ConnectQuery>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, session, mut stream) = handle(&req, stream)?;
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    let client_id: String = uuid::Uuid::new_v4().into();

    if let Some(room) = app_state
        .rooms
        .lock()
        .unwrap()
        .iter()
        .find(|room| room.platform_name == query.platform)
    {
        room.clients.lock().unwrap().insert(client_id.clone(), tx);
    } else {
        return Ok(HttpResponse::BadRequest().json(json!({
            "Status":404,
            "Message": format!("platform does not exists")
        })));
    }

    // accept connections from client and handle the connection and closing connection
    {
        rt::spawn({
            let mut session = session.clone();
            let app_state_clone = app_state.clone();
            let client_id_cloned = client_id.clone();
            let platform_name = query.0.platform.clone();

            async move {
                // receive messages from websocket
                while let Some(msg) = stream.recv().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            // echo text message
                            session.text(text).await.unwrap();
                        }

                        Ok(Message::Close(_)) => {
                            println!(" websocket got closed");
                            break;
                        }

                        Err(_) => {
                            break;
                        }
                        _ => {}
                    }
                }
                session.close(None).await.ok();
                drop(stream);

                remove_client(&app_state_clone, platform_name, client_id_cloned);
            }
        });
    }

    // send to the session client whenever any msg is pushed to the channel
    {
        rt::spawn({
            let mut session = session.clone();
            let app_state_clone = app_state.clone();
            let client_id_cloned = client_id.clone();
            let platform_name = query.0.platform.clone();
            async move {
                while let Some(brdcst_msg) = rx.recv().await {
                    if session.text(brdcst_msg).await.is_err() {
                        break;
                    }
                }
                session.close(None).await.ok();
                rx.close();
                drop(rx);

                remove_client(&app_state_clone, platform_name, client_id_cloned);
            }
        });
    }

    // ping the user every 30 seconds to check the availability. If ping failed then close the connection
    {
        rt::spawn({
            let mut session = session.clone();
            let app_state_clone = app_state.clone();
            let client_id_cloned = client_id.clone();
            let platform_name = query.0.platform.clone();
            async move {
                loop {
                    actix_web::rt::time::sleep(Duration::from_secs(30)).await;
                    if session.ping(b"").await.is_err() {
                        break;
                    }
                }
                session.close(None).await.ok();

                remove_client(&app_state_clone, platform_name, client_id_cloned);
            }
        });
    }

    // println!("client {} connected with websocket", { client_id });

    Ok(res)
}
