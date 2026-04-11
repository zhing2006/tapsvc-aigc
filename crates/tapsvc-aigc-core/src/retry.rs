use std::future::Future;
use std::time::Duration;

use rand::RngExt;
use tracing::warn;

/// Configuration for retry with exponential backoff.
///
/// `max_retries` is the number of retries (not total attempts).
/// Total attempts = 1 (initial) + max_retries.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub base_delay: Duration,
    pub factor: u32,
    pub max_jitter: Duration,
    pub max_retries: u32,
    pub max_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            base_delay: Duration::from_secs(2),
            factor: 2,
            max_jitter: Duration::from_secs(1),
            max_retries: 3,
            max_delay: Duration::from_secs(30),
        }
    }
}

impl RetryConfig {
    fn delay_for(&self, attempt: u32, retry_after: Option<Duration>) -> Duration {
        if let Some(ra) = retry_after {
            return ra;
        }

        let exp = self.factor.saturating_pow(attempt);
        let base = self.base_delay.saturating_mul(exp);

        let jitter = if self.max_jitter.is_zero() {
            Duration::ZERO
        } else {
            let jitter_ms = rand::rng().random_range(0..=self.max_jitter.as_millis() as u64);
            Duration::from_millis(jitter_ms)
        };

        let total = base.saturating_add(jitter);
        total.min(self.max_delay)
    }
}

/// Trait for errors that can indicate whether a retry is appropriate.
pub trait Retryable {
    fn is_retryable(&self) -> bool;
    fn retry_after(&self) -> Option<Duration>;
}

/// Returns `true` if the HTTP status code is retryable.
pub fn is_retryable_status(status: u16) -> bool {
    matches!(status, 429 | 500 | 502 | 503 | 504)
}

/// Generic retry executor with exponential backoff + jitter.
///
/// Calls `operation` up to `1 + config.max_retries` times.
/// On retryable errors, waits with exponential backoff before retrying.
/// On non-retryable errors, returns immediately.
pub async fn retry<F, Fut, T, E>(config: &RetryConfig, operation: F) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: Retryable + std::fmt::Display,
{
    let mut last_err;

    match operation().await {
        Ok(value) => return Ok(value),
        Err(e) => {
            if !e.is_retryable() || config.max_retries == 0 {
                return Err(e);
            }
            last_err = e;
        }
    }

    for attempt in 0..config.max_retries {
        let delay = config.delay_for(attempt, last_err.retry_after());
        warn!(
            attempt = attempt + 1,
            max_retries = config.max_retries,
            delay_ms = delay.as_millis() as u64,
            error = %last_err,
            "retrying after error"
        );
        tokio::time::sleep(delay).await;

        match operation().await {
            Ok(value) => return Ok(value),
            Err(e) => {
                if !e.is_retryable() {
                    return Err(e);
                }
                last_err = e;
            }
        }
    }

    Err(last_err)
}
