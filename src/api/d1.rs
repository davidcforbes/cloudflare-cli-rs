use serde::{Deserialize, Serialize};

/// D1 Database representation from Cloudflare API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct D1Database {
    /// Unique identifier for the database
    pub uuid: String,
    /// Name of the database
    pub name: String,
    /// Version of the database (alpha, beta, etc.)
    #[serde(default)]
    pub version: String,
    /// Number of tables in the database
    #[serde(default)]
    pub num_tables: u32,
    /// Size of the database file in bytes
    #[serde(default)]
    pub file_size: u64,
    /// When the database was created
    #[serde(default)]
    pub created_at: String,
}

/// Request payload for creating a D1 database
#[derive(Debug, Clone, Serialize)]
pub struct CreateD1Database {
    /// Name for the new database
    pub name: String,
    /// Optional location hint for the database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_location_hint: Option<String>,
}

/// Request payload for updating a D1 database
#[derive(Debug, Clone, Serialize)]
pub struct UpdateD1Database {
    /// New name for the database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// SQL query to execute against a D1 database
#[derive(Debug, Clone, Serialize)]
pub struct D1Query {
    /// SQL statement to execute
    pub sql: String,
    /// Parameters for the query (for parameterized queries)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<serde_json::Value>>,
}

/// Result of a D1 query execution
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct D1QueryResult {
    /// Query results as objects
    #[serde(default)]
    pub results: Vec<serde_json::Value>,
    /// Whether the query was successful
    #[serde(default)]
    pub success: bool,
    /// Metadata about the query execution
    #[serde(default)]
    pub meta: D1QueryMeta,
}

/// Metadata about query execution
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct D1QueryMeta {
    /// Time taken to execute the query in milliseconds
    #[serde(default)]
    pub duration: f64,
    /// Number of rows read
    #[serde(default)]
    pub rows_read: u64,
    /// Number of rows written
    #[serde(default)]
    pub rows_written: u64,
    /// Last row ID (for INSERT operations)
    #[serde(default)]
    pub last_row_id: i64,
    /// Number of changes made
    #[serde(default)]
    pub changes: u64,
    /// Size change in bytes
    #[serde(default)]
    pub size_after: u64,
    /// Whether the result was served from cache
    #[serde(default)]
    pub served_by_cache: bool,
}

/// Raw query result (array format for performance)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct D1RawQueryResult {
    /// Column names
    #[serde(default)]
    pub columns: Vec<String>,
    /// Row data as arrays
    #[serde(default)]
    pub rows: Vec<Vec<serde_json::Value>>,
    /// Whether the query was successful
    #[serde(default)]
    pub success: bool,
    /// Metadata about the query execution
    #[serde(default)]
    pub meta: D1QueryMeta,
}

/// Request to export a D1 database
#[derive(Debug, Clone, Serialize)]
pub struct D1ExportRequest {
    /// Output format (currently only "polling" is supported)
    pub output_format: String,
}

/// Response from initiating a D1 export
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct D1ExportResponse {
    /// Task ID for polling export status
    #[serde(default)]
    pub task_id: String,
    /// Status of the export task
    #[serde(default)]
    pub status: String,
    /// Signed URL to download the export (when complete)
    #[serde(default)]
    pub signed_url: Option<String>,
    /// Error message if export failed
    #[serde(default)]
    pub error: Option<String>,
}

/// Request to import SQL into a D1 database
#[derive(Debug, Clone, Serialize)]
pub struct D1ImportRequest {
    /// URL to the SQL file to import
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// SQL content to import directly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql: Option<String>,
}

/// Response from a D1 import operation
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct D1ImportResponse {
    /// Number of queries executed
    #[serde(default)]
    pub num_queries: u64,
    /// Whether the import was successful
    #[serde(default)]
    pub success: bool,
    /// Error message if import failed
    #[serde(default)]
    pub error: Option<String>,
}

/// Time travel bookmark for point-in-time recovery
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct D1Bookmark {
    /// Bookmark identifier
    #[serde(default)]
    pub bookmark: String,
    /// Timestamp of the bookmark
    #[serde(default)]
    pub timestamp: String,
}

/// Request to restore a D1 database to a point in time
#[derive(Debug, Clone, Serialize)]
pub struct D1RestoreRequest {
    /// Bookmark to restore to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmark: Option<String>,
    /// Timestamp to restore to (alternative to bookmark)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// Response from a D1 restore operation
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct D1RestoreResponse {
    /// Whether the restore was successful
    #[serde(default)]
    pub success: bool,
    /// Bookmark of the restored state
    #[serde(default)]
    pub bookmark: String,
    /// Number of queries replayed
    #[serde(default)]
    pub num_queries_replayed: u64,
}
