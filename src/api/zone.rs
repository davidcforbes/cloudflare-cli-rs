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
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub original_name_servers: Vec<String>,
    pub owner: Owner,
    pub account: Account,
    pub created_on: String,
    pub modified_on: String,
}

/// Deserialize null as default value (empty vec, etc.)
fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::deserialize(deserializer)?.unwrap_or_default())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zone_deserialize_with_null_owner_id() {
        let json = r#"{
            "id": "023e105f4ecef8ad9ca31a8372d0c353",
            "name": "example.com",
            "status": "active",
            "paused": false,
            "development_mode": 0,
            "name_servers": ["ns1.cloudflare.com"],
            "original_name_servers": null,
            "owner": {
                "id": null,
                "type": "user",
                "email": "user@example.com"
            },
            "account": {
                "id": "account123",
                "name": "Test Account"
            },
            "created_on": "2024-01-01T00:00:00Z",
            "modified_on": "2024-01-01T00:00:00Z"
        }"#;

        let zone: Zone =
            serde_json::from_str(json).expect("Failed to deserialize zone with null owner.id");
        assert_eq!(zone.id, "023e105f4ecef8ad9ca31a8372d0c353");
        assert_eq!(zone.name, "example.com");
        assert!(zone.owner.id.is_none());
        assert!(zone.original_name_servers.is_empty());
    }

    #[test]
    fn test_zone_deserialize_with_all_values() {
        let json = r#"{
            "id": "zone123",
            "name": "test.com",
            "status": "pending",
            "paused": true,
            "development_mode": 1,
            "name_servers": ["ns1.cf.com", "ns2.cf.com"],
            "original_name_servers": ["ns1.original.com"],
            "owner": {
                "id": "owner123",
                "type": "organization",
                "email": "admin@org.com"
            },
            "account": {
                "id": "acc123",
                "name": "Org Account"
            },
            "created_on": "2024-01-01T00:00:00Z",
            "modified_on": "2024-01-02T00:00:00Z"
        }"#;

        let zone: Zone = serde_json::from_str(json).expect("Failed to deserialize zone");
        assert_eq!(zone.id, "zone123");
        assert!(zone.paused);
        assert_eq!(zone.owner.id, Some("owner123".to_string()));
        assert_eq!(zone.original_name_servers.len(), 1);
    }

    #[test]
    fn test_zone_deserialize_missing_optional_fields() {
        let json = r#"{
            "id": "zone123",
            "name": "test.com",
            "status": "active",
            "paused": false,
            "development_mode": 0,
            "name_servers": [],
            "owner": {
                "type": "user",
                "email": null
            },
            "account": {
                "id": "acc123",
                "name": "Account"
            },
            "created_on": "2024-01-01T00:00:00Z",
            "modified_on": "2024-01-01T00:00:00Z"
        }"#;

        let zone: Zone =
            serde_json::from_str(json).expect("Failed to deserialize zone with missing fields");
        assert!(zone.owner.id.is_none());
        assert!(zone.owner.email.is_none());
        assert!(zone.original_name_servers.is_empty());
    }

    #[test]
    fn test_owner_deserialize_null_email() {
        let json = r#"{
            "id": "owner123",
            "type": "user",
            "email": null
        }"#;

        let owner: Owner = serde_json::from_str(json).expect("Failed to deserialize owner");
        assert_eq!(owner.id, Some("owner123".to_string()));
        assert!(owner.email.is_none());
    }

    #[test]
    fn test_zone_settings_serialize_skips_none() {
        let settings = ZoneSettings {
            security_level: Some("high".to_string()),
            cache_level: None,
            development_mode: None,
            ipv6: None,
            ssl: None,
            always_use_https: None,
            minify: None,
        };

        let json = serde_json::to_string(&settings).expect("Failed to serialize");
        assert!(json.contains("security_level"));
        assert!(!json.contains("cache_level"));
    }
}
