use crate::api::d1::{
    CreateD1Database, D1Bookmark, D1Database, D1ExportRequest, D1ExportResponse, D1ImportRequest,
    D1ImportResponse, D1Query, D1QueryResult, D1RawQueryResult, D1RestoreRequest,
    D1RestoreResponse, UpdateD1Database,
};
use crate::client::{CfResponse, CloudflareClient};
use crate::error::Result;

/// List all D1 databases for an account
pub async fn list_databases(
    client: &CloudflareClient,
    account_id: &str,
) -> Result<Vec<D1Database>> {
    let endpoint = format!("/accounts/{}/d1/database", account_id);
    let response: CfResponse<Vec<D1Database>> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or_default())
}

/// Get a specific D1 database by ID
pub async fn get_database(
    client: &CloudflareClient,
    account_id: &str,
    database_id: &str,
) -> Result<D1Database> {
    let endpoint = format!("/accounts/{}/d1/database/{}", account_id, database_id);
    let response: CfResponse<D1Database> = client.get(&endpoint).await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::not_found("D1 database", database_id))
}

/// Create a new D1 database
pub async fn create_database(
    client: &CloudflareClient,
    account_id: &str,
    database: CreateD1Database,
) -> Result<D1Database> {
    let endpoint = format!("/accounts/{}/d1/database", account_id);
    let response: CfResponse<D1Database> = client.post(&endpoint, database).await?;
    let db = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from create database"))?;
    println!("✓ Created D1 database: {}", db.name);
    Ok(db)
}

/// Update a D1 database (full update)
pub async fn update_database(
    client: &CloudflareClient,
    account_id: &str,
    database_id: &str,
    update: UpdateD1Database,
) -> Result<D1Database> {
    let endpoint = format!("/accounts/{}/d1/database/{}", account_id, database_id);
    let response: CfResponse<D1Database> = client.put(&endpoint, update).await?;
    let db = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from update database"))?;
    println!("✓ Updated D1 database: {}", db.name);
    Ok(db)
}

/// Delete a D1 database
pub async fn delete_database(
    client: &CloudflareClient,
    account_id: &str,
    database_id: &str,
) -> Result<()> {
    let endpoint = format!("/accounts/{}/d1/database/{}", account_id, database_id);
    let _response: CfResponse<serde_json::Value> = client.delete(&endpoint).await?;
    println!("✓ Deleted D1 database");
    Ok(())
}

/// Execute a SQL query against a D1 database (returns objects)
pub async fn query_database(
    client: &CloudflareClient,
    account_id: &str,
    database_id: &str,
    sql: &str,
    params: Option<Vec<serde_json::Value>>,
) -> Result<Vec<D1QueryResult>> {
    let endpoint = format!("/accounts/{}/d1/database/{}/query", account_id, database_id);
    let query = D1Query {
        sql: sql.to_string(),
        params,
    };
    let response: CfResponse<Vec<D1QueryResult>> = client.post(&endpoint, query).await?;
    Ok(response.result.unwrap_or_default())
}

/// Execute a SQL query against a D1 database (returns arrays for performance)
pub async fn query_database_raw(
    client: &CloudflareClient,
    account_id: &str,
    database_id: &str,
    sql: &str,
    params: Option<Vec<serde_json::Value>>,
) -> Result<Vec<D1RawQueryResult>> {
    let endpoint = format!("/accounts/{}/d1/database/{}/raw", account_id, database_id);
    let query = D1Query {
        sql: sql.to_string(),
        params,
    };
    let response: CfResponse<Vec<D1RawQueryResult>> = client.post(&endpoint, query).await?;
    Ok(response.result.unwrap_or_default())
}

/// Export a D1 database to SQL
pub async fn export_database(
    client: &CloudflareClient,
    account_id: &str,
    database_id: &str,
) -> Result<D1ExportResponse> {
    let endpoint = format!(
        "/accounts/{}/d1/database/{}/export",
        account_id, database_id
    );
    let request = D1ExportRequest {
        output_format: "polling".to_string(),
    };
    let response: CfResponse<D1ExportResponse> = client.post(&endpoint, request).await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from export database"))
}

/// Import SQL into a D1 database
pub async fn import_database(
    client: &CloudflareClient,
    account_id: &str,
    database_id: &str,
    sql: &str,
) -> Result<D1ImportResponse> {
    let endpoint = format!(
        "/accounts/{}/d1/database/{}/import",
        account_id, database_id
    );
    let request = D1ImportRequest {
        sql: Some(sql.to_string()),
        url: None,
    };
    let response: CfResponse<D1ImportResponse> = client.post(&endpoint, request).await?;
    let result = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from import database"))?;
    println!("✓ Imported {} queries into database", result.num_queries);
    Ok(result)
}

/// Get the current time travel bookmark for a D1 database
pub async fn get_bookmark(
    client: &CloudflareClient,
    account_id: &str,
    database_id: &str,
    timestamp: Option<&str>,
) -> Result<D1Bookmark> {
    let mut endpoint = format!(
        "/accounts/{}/d1/database/{}/time_travel/bookmark",
        account_id, database_id
    );
    if let Some(ts) = timestamp {
        endpoint.push_str(&format!("?timestamp={}", ts));
    }
    let response: CfResponse<D1Bookmark> = client.get(&endpoint).await?;
    response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No bookmark returned"))
}

/// Restore a D1 database to a previous point in time
pub async fn restore_database(
    client: &CloudflareClient,
    account_id: &str,
    database_id: &str,
    bookmark: Option<&str>,
    timestamp: Option<&str>,
) -> Result<D1RestoreResponse> {
    let endpoint = format!(
        "/accounts/{}/d1/database/{}/time_travel/restore",
        account_id, database_id
    );
    let request = D1RestoreRequest {
        bookmark: bookmark.map(String::from),
        timestamp: timestamp.map(String::from),
    };
    let response: CfResponse<D1RestoreResponse> = client.post(&endpoint, request).await?;
    let result = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from restore"))?;
    println!(
        "✓ Restored database ({} queries replayed)",
        result.num_queries_replayed
    );
    Ok(result)
}
