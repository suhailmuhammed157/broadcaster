use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct AddPlatform {
    pub platform_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Broadcast {
    pub message: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct RenewPlatformToken {
    pub platform_id: String,
    pub platform_name: String,
}
