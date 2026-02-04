//! Cloudflare API Token models
//!
//! Models for the Cloudflare Token API endpoints.

use serde::{Deserialize, Serialize};

/// Represents a Cloudflare API token
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token {
    pub id: String,
    pub name: String,
    pub status: String,
    #[serde(default)]
    pub issued_on: Option<String>,
    #[serde(default)]
    pub modified_on: Option<String>,
    #[serde(default)]
    pub not_before: Option<String>,
    #[serde(default)]
    pub expires_on: Option<String>,
    #[serde(default)]
    pub last_used_on: Option<String>,
    #[serde(default)]
    pub policies: Vec<TokenPolicy>,
    #[serde(default)]
    pub condition: Option<TokenCondition>,
}

/// Token policy that defines permissions
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenPolicy {
    pub id: String,
    pub effect: String,
    pub resources: serde_json::Value,
    pub permission_groups: Vec<PermissionGroupRef>,
}

/// Reference to a permission group within a policy
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionGroupRef {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
}

/// Condition restrictions for token usage
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenCondition {
    #[serde(default)]
    pub request_ip: Option<IpCondition>,
}

/// IP address restrictions
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IpCondition {
    #[serde(default, rename = "in")]
    pub allowed: Vec<String>,
    #[serde(default)]
    pub not_in: Vec<String>,
}

/// Permission group available for tokens
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionGroup {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub scopes: Vec<String>,
}

/// Token verification response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenVerification {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub not_before: Option<String>,
    #[serde(default)]
    pub expires_on: Option<String>,
}

/// Request to create a new token
#[derive(Debug, Clone, Serialize)]
pub struct CreateToken {
    pub name: String,
    pub policies: Vec<CreateTokenPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<TokenCondition>,
}

/// Policy for token creation
#[derive(Debug, Clone, Serialize)]
pub struct CreateTokenPolicy {
    pub effect: String,
    pub resources: serde_json::Value,
    pub permission_groups: Vec<CreatePermissionGroupRef>,
}

/// Permission group reference for creation
#[derive(Debug, Clone, Serialize)]
pub struct CreatePermissionGroupRef {
    pub id: String,
}

/// Request to update a token
#[derive(Debug, Clone, Serialize)]
pub struct UpdateToken {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<CreateTokenPolicy>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<TokenCondition>,
}

/// Response when creating a token (includes the secret value)
#[derive(Debug, Clone, Deserialize)]
pub struct TokenCreateResponse {
    pub id: String,
    pub name: String,
    pub status: String,
    /// The token value - only returned on creation
    pub value: String,
    #[serde(default)]
    pub issued_on: Option<String>,
    #[serde(default)]
    pub modified_on: Option<String>,
    #[serde(default)]
    pub not_before: Option<String>,
    #[serde(default)]
    pub expires_on: Option<String>,
    #[serde(default)]
    pub policies: Vec<TokenPolicy>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_deserialize() {
        let json = r#"{
            "id": "token123",
            "name": "Test Token",
            "status": "active",
            "issued_on": "2024-01-01T00:00:00Z",
            "modified_on": "2024-01-02T00:00:00Z",
            "not_before": null,
            "expires_on": "2025-01-01T00:00:00Z",
            "last_used_on": "2024-06-15T12:00:00Z",
            "policies": []
        }"#;

        let token: Token = serde_json::from_str(json).expect("Failed to deserialize token");
        assert_eq!(token.id, "token123");
        assert_eq!(token.status, "active");
        assert!(token.not_before.is_none());
        assert!(token.expires_on.is_some());
    }

    #[test]
    fn test_token_deserialize_with_policies() {
        let json = r#"{
            "id": "token456",
            "name": "Full Token",
            "status": "active",
            "policies": [
                {
                    "id": "policy123",
                    "effect": "allow",
                    "resources": {"com.cloudflare.api.account.*": "*"},
                    "permission_groups": [
                        {"id": "perm123", "name": "Zone Read"}
                    ]
                }
            ]
        }"#;

        let token: Token =
            serde_json::from_str(json).expect("Failed to deserialize token with policies");
        assert_eq!(token.policies.len(), 1);
        assert_eq!(token.policies[0].effect, "allow");
    }

    #[test]
    fn test_permission_group_deserialize() {
        let json = r#"{
            "id": "group123",
            "name": "Zone Read",
            "description": "Read zone settings",
            "scopes": ["com.cloudflare.api.account.zone"]
        }"#;

        let group: PermissionGroup =
            serde_json::from_str(json).expect("Failed to deserialize permission group");
        assert_eq!(group.id, "group123");
        assert_eq!(group.name, "Zone Read");
    }

    #[test]
    fn test_create_token_serialize() {
        let create = CreateToken {
            name: "New Token".to_string(),
            policies: vec![CreateTokenPolicy {
                effect: "allow".to_string(),
                resources: serde_json::json!({"com.cloudflare.api.account.*": "*"}),
                permission_groups: vec![CreatePermissionGroupRef {
                    id: "perm123".to_string(),
                }],
            }],
            not_before: None,
            expires_on: Some("2025-12-31T23:59:59Z".to_string()),
            condition: None,
        };

        let json = serde_json::to_string(&create).expect("Failed to serialize");
        assert!(json.contains("New Token"));
        assert!(json.contains("2025-12-31"));
        assert!(!json.contains("not_before")); // Should be skipped
    }

    #[test]
    fn test_update_token_serialize_partial() {
        let update = UpdateToken {
            name: Some("Renamed Token".to_string()),
            status: None,
            policies: None,
            not_before: None,
            expires_on: None,
            condition: None,
        };

        let json = serde_json::to_string(&update).expect("Failed to serialize");
        assert!(json.contains("Renamed Token"));
        assert!(!json.contains("status"));
        assert!(!json.contains("policies"));
    }

    #[test]
    fn test_token_verification_deserialize() {
        let json = r#"{
            "id": "token789",
            "status": "active",
            "not_before": null,
            "expires_on": "2025-06-01T00:00:00Z"
        }"#;

        let verification: TokenVerification =
            serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(verification.status, "active");
        assert!(verification.not_before.is_none());
    }
}
