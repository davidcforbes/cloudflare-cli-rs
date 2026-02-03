use crate::error::Result;
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
        }
    }
}

pub async fn retry_with_backoff<F, Fut, T>(operation: F, config: RetryConfig) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut attempt = 1;
    let mut delay = config.initial_delay;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt >= config.max_attempts || !e.is_retryable() => {
                return Err(e);
            }
            Err(e) => {
                warn!(
                    "Attempt {}/{} failed: {}. Retrying in {:?}...",
                    attempt, config.max_attempts, e, delay
                );
                sleep(delay).await;
                attempt += 1;
                delay = std::cmp::min(
                    Duration::from_secs_f64(delay.as_secs_f64() * config.multiplier),
                    config.max_delay,
                );
            }
        }
    }
}
