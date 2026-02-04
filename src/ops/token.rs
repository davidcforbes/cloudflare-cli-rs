//! Token operations
//!
//! Functions for managing Cloudflare API tokens.

use crate::api::token::{
    CreateToken, PermissionGroup, Token, TokenCreateResponse, TokenVerification, UpdateToken,
};
use crate::client::{CfResponse, CloudflareClient};
use crate::error::Result;

/// List all API tokens for the authenticated user
pub async fn list_tokens(client: &CloudflareClient) -> Result<Vec<Token>> {
    let response: CfResponse<Vec<Token>> = client.get("/user/tokens").await?;
    Ok(response.result.unwrap_or_default())
}

/// Get a specific token by ID
pub async fn get_token(client: &CloudflareClient, token_id: &str) -> Result<Token> {
    let response: CfResponse<Token> = client.get(&format!("/user/tokens/{}", token_id)).await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::not_found("Token", token_id))
}

/// Create a new API token
pub async fn create_token(
    client: &CloudflareClient,
    token: CreateToken,
) -> Result<TokenCreateResponse> {
    let response: CfResponse<TokenCreateResponse> = client.post("/user/tokens", &token).await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from create token"))
}

/// Update an existing token
pub async fn update_token(
    client: &CloudflareClient,
    token_id: &str,
    update: UpdateToken,
) -> Result<Token> {
    let response: CfResponse<Token> = client
        .put(&format!("/user/tokens/{}", token_id), &update)
        .await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::not_found("Token", token_id))
}

/// Delete a token
pub async fn delete_token(client: &CloudflareClient, token_id: &str) -> Result<()> {
    let _response: CfResponse<serde_json::Value> =
        client.delete(&format!("/user/tokens/{}", token_id)).await?;
    println!("✓ Deleted token");
    Ok(())
}

/// Verify the current token's validity
pub async fn verify_token(client: &CloudflareClient) -> Result<TokenVerification> {
    let response: CfResponse<TokenVerification> = client.get("/user/tokens/verify").await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::api("Token verification failed"))
}

/// List all available permission groups
pub async fn list_permission_groups(client: &CloudflareClient) -> Result<Vec<PermissionGroup>> {
    let response: CfResponse<Vec<PermissionGroup>> =
        client.get("/user/tokens/permission_groups").await?;
    Ok(response.result.unwrap_or_default())
}

/// Roll (regenerate) a token's value
pub async fn roll_token(client: &CloudflareClient, token_id: &str) -> Result<TokenCreateResponse> {
    // Empty body for PUT to roll endpoint
    let empty: serde_json::Value = serde_json::json!({});
    let response: CfResponse<TokenCreateResponse> = client
        .put(&format!("/user/tokens/{}/value", token_id), &empty)
        .await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::api("Failed to roll token"))
}
