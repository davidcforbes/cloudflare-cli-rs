use crate::config::AuthMethod;
use crate::error::{CfadError, Result};
use reqwest::header::{self, HeaderMap};
use reqwest::{Client as HttpClient, Method};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

pub mod retry;
pub use retry::{retry_with_backoff, RetryConfig};

#[derive(Debug)]
pub struct CloudflareClient {
    http_client: HttpClient,
    #[allow(dead_code)] // Used for debugging and potential future features
    auth: AuthMethod,
    base_url: String,
    rate_limiter: Arc<Semaphore>,
}

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

impl CloudflareClient {
    pub fn new(auth: AuthMethod) -> Result<Self> {
        Self::new_with_base_url(auth, "https://api.cloudflare.com/client/v4".to_string())
    }

    pub fn new_with_base_url(auth: AuthMethod, base_url: String) -> Result<Self> {
        let mut headers = HeaderMap::new();

        match &auth {
            AuthMethod::ApiToken(token) => {
                headers.insert(
                    "Authorization",
                    header::HeaderValue::from_str(&format!("Bearer {}", token))
                        .map_err(|_| CfadError::auth("Invalid API token"))?,
                );
            }
            AuthMethod::ApiKeyEmail { key, email } => {
                headers.insert(
                    "X-Auth-Key",
                    header::HeaderValue::from_str(key)
                        .map_err(|_| CfadError::auth("Invalid API key"))?,
                );
                headers.insert(
                    "X-Auth-Email",
                    header::HeaderValue::from_str(email)
                        .map_err(|_| CfadError::auth("Invalid email"))?,
                );
            }
        }

        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert("User-Agent", header::HeaderValue::from_static("cfad/0.2.0"));

        let http_client = HttpClient::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            http_client,
            auth,
            base_url,
            rate_limiter: Arc::new(Semaphore::new(4)), // 4 req/s default
        })
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(&self, endpoint: &str) -> Result<CfResponse<T>> {
        self.request(Method::GET, endpoint, None::<()>).await
    }

    pub async fn post<B: Serialize, T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        body: B,
    ) -> Result<CfResponse<T>> {
        self.request(Method::POST, endpoint, Some(body)).await
    }

    pub async fn put<B: Serialize, T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        body: B,
    ) -> Result<CfResponse<T>> {
        self.request(Method::PUT, endpoint, Some(body)).await
    }

    pub async fn patch<B: Serialize, T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        body: B,
    ) -> Result<CfResponse<T>> {
        self.request(Method::PATCH, endpoint, Some(body)).await
    }

    pub async fn delete<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
    ) -> Result<CfResponse<T>> {
        self.request(Method::DELETE, endpoint, None::<()>).await
    }

    async fn request<B: Serialize, T: for<'de> Deserialize<'de>>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<B>,
    ) -> Result<CfResponse<T>> {
        // Rate limiting
        let _permit = self
            .rate_limiter
            .acquire()
            .await
            .map_err(|_| CfadError::network("Rate limiter failed"))?;

        let url = format!("{}{}", self.base_url, endpoint);

        let mut request = self.http_client.request(method, &url);

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(CfadError::Api {
                status: status.as_u16(),
                message: text,
                code: None,
            });
        }

        // Get response text first for better error messages
        let text = response.text().await?;
        let cf_response: CfResponse<T> = serde_json::from_str(&text).map_err(|e| {
            // Log the first 500 chars of response for debugging
            let preview = if text.len() > 500 {
                format!("{}...", &text[..500])
            } else {
                text.clone()
            };
            log::debug!("Failed to parse response: {}", preview);
            CfadError::Other(format!("JSON parse error: {} - Response: {}", e, preview))
        })?;

        if !cf_response.success {
            return Err(CfadError::from_cf_errors(
                cf_response
                    .errors
                    .into_iter()
                    .map(|e| crate::error::CfError {
                        code: e.code,
                        message: e.message,
                    })
                    .collect(),
            ));
        }

        Ok(cf_response)
    }
}
