use std::future::{Ready, ready};

use actix_web::{
    FromRequest, HttpRequest, HttpResponse, dev::Payload, error::InternalError, http::header,
};
use jwt_lib::Platform;
use serde_json::json;

#[derive(Debug)]
pub struct AuthPlatform(pub Platform);

impl FromRequest for AuthPlatform {
    type Error = InternalError<String>;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let access_token = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|str| str.split(" ").nth(1));

        match access_token {
            Some(token) => {
                let platform = jwt_lib::decode_jwt(token);

                match platform {
                    Ok(platform) => ready(Ok(AuthPlatform(platform))),

                    Err(e) => ready(Err(InternalError::from_response(
                        e.clone(),
                        HttpResponse::Unauthorized().json(json!({
                          "success": false,
                            "message": "error or invalid token"
                        })),
                    ))),
                }
            }

            None => ready(Err(InternalError::from_response(
                String::from("No token provided"),
                HttpResponse::Unauthorized().json(json!({
                  "success": false,
                    "message": "No token provided"

                })),
            ))),
        }
    }
}
