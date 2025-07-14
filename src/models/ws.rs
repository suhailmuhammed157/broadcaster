use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConnectQuery {
    pub platform: String,
}
