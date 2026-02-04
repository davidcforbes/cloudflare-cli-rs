use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Zone {
    pub id: String,
    pub name: String,
    pub status: String,
    pub paused: bool,
    pub development_mode: u32,
    #[serde(default)]
    pub name_servers: Vec<String>,
    #[serde(default)]
    pub original_name_servers: Vec<String>,
    pub owner: Owner,
    pub account: Account,
    pub created_on: String,
    pub modified_on: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Owner {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub owner_type: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ZoneSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_level: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_level: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub development_mode: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_use_https: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minify: Option<MinifySettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinifySettings {
    pub css: bool,
    pub html: bool,
    pub js: bool,
}
