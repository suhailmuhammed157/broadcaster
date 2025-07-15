use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddPlatform {
    pub platform_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Broadcast {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct RenewPlatformToken {
    pub platform_id: String,
    pub platform_name: String,
}
