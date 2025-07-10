use std::time::Duration;

use actix_web::{Error, HttpRequest, HttpResponse, get, post, rt, web};
use actix_ws::{Message, handle};
use serde_json::json;
use tokio::sync::mpsc;

use crate::AppState;

#[get("")]
pub async fn connect(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, session, mut stream) = handle(&req, stream)?;
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    let client_id: String = uuid::Uuid::new_v4().into();
    app_state.clients.lock().unwrap().insert(client_id, tx);

    // accept connections from client and handle the connection and closing connection
    {
        rt::spawn({
            let mut session = session.clone();
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
            }
        });
    }

    // ping the user every 30 seconds to check the availability. If ping failed then close the connection
    {
        rt::spawn({
            let mut session = session.clone();
            async move {
                loop {
                    actix_web::rt::time::sleep(Duration::from_secs(30)).await;
                    if session.ping(b"").await.is_err() {
                        break;
                    }
                }
                session.close(None).await.ok();
            }
        });
    }

    Ok(res)
}
