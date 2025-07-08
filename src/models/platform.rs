use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddPlatform {
    pub platform_name: String,
}
