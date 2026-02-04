use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DnsRecord {
    pub id: String,
    pub zone_id: String,
    pub zone_name: String,
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub content: String,
    pub ttl: u32,
    pub proxied: bool,
    pub locked: bool,
    pub created_on: String,
    pub modified_on: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateDnsRecord {
    #[serde(rename = "type")]
    pub record_type: String,
    pub name: String,
    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateDnsRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub record_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u16>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_record_deserialize_a_record() {
        let json = r#"{
            "id": "372e67954025e0ba6aaa6d586b9e0b59",
            "zone_id": "023e105f4ecef8ad9ca31a8372d0c353",
            "zone_name": "example.com",
            "name": "example.com",
            "type": "A",
            "content": "198.51.100.4",
            "proxied": true,
            "ttl": 1,
            "locked": false,
            "created_on": "2024-01-01T00:00:00Z",
            "modified_on": "2024-01-01T00:00:00Z"
        }"#;

        let record: DnsRecord = serde_json::from_str(json).expect("Failed to deserialize A record");
        assert_eq!(record.id, "372e67954025e0ba6aaa6d586b9e0b59");
        assert_eq!(record.record_type, "A");
        assert_eq!(record.content, "198.51.100.4");
        assert!(record.proxied);
        assert_eq!(record.ttl, 1);
        assert!(record.priority.is_none());
    }

    #[test]
    fn test_dns_record_deserialize_mx_record_with_priority() {
        let json = r#"{
            "id": "mx123",
            "zone_id": "zone123",
            "zone_name": "example.com",
            "name": "mail.example.com",
            "type": "MX",
            "content": "mx.example.com",
            "proxied": false,
            "ttl": 3600,
            "locked": false,
            "priority": 10,
            "created_on": "2024-01-01T00:00:00Z",
            "modified_on": "2024-01-01T00:00:00Z"
        }"#;

        let record: DnsRecord =
            serde_json::from_str(json).expect("Failed to deserialize MX record");
        assert_eq!(record.record_type, "MX");
        assert_eq!(record.priority, Some(10));
        assert!(!record.proxied);
    }

    #[test]
    fn test_dns_record_deserialize_with_null_data() {
        let json = r#"{
            "id": "txt123",
            "zone_id": "zone123",
            "zone_name": "example.com",
            "name": "_dmarc.example.com",
            "type": "TXT",
            "content": "v=DMARC1; p=none",
            "proxied": false,
            "ttl": 1,
            "locked": false,
            "data": null,
            "created_on": "2024-01-01T00:00:00Z",
            "modified_on": "2024-01-01T00:00:00Z"
        }"#;

        let record: DnsRecord =
            serde_json::from_str(json).expect("Failed to deserialize TXT record with null data");
        assert_eq!(record.record_type, "TXT");
        assert!(record.data.is_none());
    }

    #[test]
    fn test_dns_record_deserialize_with_data_object() {
        let json = r#"{
            "id": "srv123",
            "zone_id": "zone123",
            "zone_name": "example.com",
            "name": "_sip._tcp.example.com",
            "type": "SRV",
            "content": "10 5 5060 sip.example.com",
            "proxied": false,
            "ttl": 3600,
            "locked": false,
            "priority": 10,
            "data": {
                "priority": 10,
                "weight": 5,
                "port": 5060,
                "target": "sip.example.com"
            },
            "created_on": "2024-01-01T00:00:00Z",
            "modified_on": "2024-01-01T00:00:00Z"
        }"#;

        let record: DnsRecord =
            serde_json::from_str(json).expect("Failed to deserialize SRV record");
        assert_eq!(record.record_type, "SRV");
        assert!(record.data.is_some());
    }

    #[test]
    fn test_create_dns_record_serialize() {
        let record = CreateDnsRecord {
            record_type: "A".to_string(),
            name: "www".to_string(),
            content: "192.0.2.1".to_string(),
            ttl: Some(3600),
            proxied: Some(true),
            priority: None,
            data: None,
        };

        let json = serde_json::to_string(&record).expect("Failed to serialize");
        assert!(json.contains(r#""type":"A""#));
        assert!(json.contains(r#""name":"www""#));
        assert!(!json.contains("priority"));
    }

    #[test]
    fn test_update_dns_record_serialize_partial() {
        let update = UpdateDnsRecord {
            record_type: None,
            name: None,
            content: Some("203.0.113.1".to_string()),
            ttl: None,
            proxied: Some(false),
            priority: None,
        };

        let json = serde_json::to_string(&update).expect("Failed to serialize");
        assert!(json.contains("content"));
        assert!(json.contains("proxied"));
        assert!(!json.contains("name"));
        assert!(!json.contains("ttl"));
    }
}
