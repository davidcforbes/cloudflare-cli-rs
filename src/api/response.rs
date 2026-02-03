use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CfResponse<T> {
    pub success: bool,
    pub errors: Vec<CfApiError>,
    pub messages: Vec<CfMessage>,
    pub result: Option<T>,
    pub result_info: Option<ResultInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CfApiError {
    pub code: u32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CfMessage {
    pub code: u32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultInfo {
    pub page: u32,
    pub per_page: u32,
    pub count: u32,
    pub total_count: u32,
    pub total_pages: u32,
}
