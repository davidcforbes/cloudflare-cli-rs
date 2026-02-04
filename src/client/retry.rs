use crate::error::Result;
use log::warn;
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CfadError;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_retry_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.initial_delay, Duration::from_millis(100));
        assert_eq!(config.max_delay, Duration::from_secs(30));
        assert_eq!(config.multiplier, 2.0);
    }

    #[tokio::test]
    async fn test_retry_succeeds_first_attempt() {
        let config = RetryConfig::default();
        let result = retry_with_backoff(|| async { Ok::<i32, CfadError>(42) }, config).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_retry_succeeds_after_failures() {
        let config = RetryConfig {
            max_attempts: 3,
            initial_delay: Duration::from_millis(1),
            max_delay: Duration::from_secs(1),
            multiplier: 2.0,
        };

        let attempt_count = Arc::new(AtomicU32::new(0));
        let attempt_count_clone = attempt_count.clone();

        let result = retry_with_backoff(
            move || {
                let count = attempt_count_clone.clone();
                async move {
                    let current = count.fetch_add(1, Ordering::SeqCst) + 1;
                    if current < 3 {
                        Err(CfadError::network("Connection failed"))
                    } else {
                        Ok(42)
                    }
                }
            },
            config,
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempt_count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_max_attempts_reached() {
        let config = RetryConfig {
            max_attempts: 2,
            initial_delay: Duration::from_millis(1),
            max_delay: Duration::from_secs(1),
            multiplier: 2.0,
        };

        let attempt_count = Arc::new(AtomicU32::new(0));
        let attempt_count_clone = attempt_count.clone();

        let result = retry_with_backoff(
            move || {
                let count = attempt_count_clone.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    Err::<i32, CfadError>(CfadError::network("Always fails"))
                }
            },
            config,
        )
        .await;

        assert!(result.is_err());
        assert_eq!(attempt_count.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_retry_non_retryable_error_stops_immediately() {
        let config = RetryConfig {
            max_attempts: 3,
            initial_delay: Duration::from_millis(1),
            max_delay: Duration::from_secs(1),
            multiplier: 2.0,
        };

        let attempt_count = Arc::new(AtomicU32::new(0));
        let attempt_count_clone = attempt_count.clone();

        let result = retry_with_backoff(
            move || {
                let count = attempt_count_clone.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    Err::<i32, CfadError>(CfadError::auth("Invalid credentials"))
                }
            },
            config,
        )
        .await;

        assert!(result.is_err());
        assert_eq!(attempt_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_retry_exponential_backoff() {
        let config = RetryConfig {
            max_attempts: 4,
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_secs(1),
            multiplier: 2.0,
        };

        let attempt_count = Arc::new(AtomicU32::new(0));
        let attempt_count_clone = attempt_count.clone();

        let start = std::time::Instant::now();
        let result = retry_with_backoff(
            move || {
                let count = attempt_count_clone.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    Err::<i32, CfadError>(CfadError::Timeout(Duration::from_secs(1)))
                }
            },
            config,
        )
        .await;

        let elapsed = start.elapsed();

        assert!(result.is_err());
        assert_eq!(attempt_count.load(Ordering::SeqCst), 4);
        // Should wait at least: 10ms + 20ms + 40ms = 70ms
        assert!(elapsed >= Duration::from_millis(70));
    }

    #[tokio::test]
    async fn test_retry_max_delay_cap() {
        let config = RetryConfig {
            max_attempts: 5,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_millis(150), // Cap at 150ms
            multiplier: 2.0,
        };

        let attempt_count = Arc::new(AtomicU32::new(0));
        let attempt_count_clone = attempt_count.clone();

        let start = std::time::Instant::now();
        let result = retry_with_backoff(
            move || {
                let count = attempt_count_clone.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    Err::<i32, CfadError>(CfadError::RateLimit { retry_after: None })
                }
            },
            config,
        )
        .await;

        let elapsed = start.elapsed();

        assert!(result.is_err());
        assert_eq!(attempt_count.load(Ordering::SeqCst), 5);
        // Delays: 100ms, 150ms (capped), 150ms (capped), 150ms (capped) = 550ms total
        assert!(elapsed >= Duration::from_millis(550));
        // Should not exceed much more due to cap
        assert!(elapsed < Duration::from_millis(800));
    }

    #[tokio::test]
    async fn test_retry_network_error_is_retryable() {
        let config = RetryConfig {
            max_attempts: 2,
            initial_delay: Duration::from_millis(1),
            max_delay: Duration::from_secs(1),
            multiplier: 2.0,
        };

        let result = retry_with_backoff(
            || async { Err::<i32, CfadError>(CfadError::network("Network error")) },
            config,
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_retry_timeout_error_is_retryable() {
        let config = RetryConfig {
            max_attempts: 2,
            initial_delay: Duration::from_millis(1),
            max_delay: Duration::from_secs(1),
            multiplier: 2.0,
        };

        let result = retry_with_backoff(
            || async { Err::<i32, CfadError>(CfadError::Timeout(Duration::from_secs(30))) },
            config,
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_retry_rate_limit_error_is_retryable() {
        let config = RetryConfig {
            max_attempts: 2,
            initial_delay: Duration::from_millis(1),
            max_delay: Duration::from_secs(1),
            multiplier: 2.0,
        };

        let result = retry_with_backoff(
            || async {
                Err::<i32, CfadError>(CfadError::RateLimit {
                    retry_after: Some(Duration::from_secs(60)),
                })
            },
            config,
        )
        .await;

        assert!(result.is_err());
    }
}
