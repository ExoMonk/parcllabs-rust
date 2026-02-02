//! Shared fetch helpers with retry logic for GET and batch POST endpoints.

use crate::error::{ParclError, Result};
use crate::models::{BatchMetricsResponse, MetricsResponse};
use crate::RetryConfig;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::time::Duration;

/// Executes a single GET request with retry on 429.
pub(crate) async fn get_page<T: DeserializeOwned>(
    http: &Client,
    api_key: &str,
    url: &str,
    retry_config: &RetryConfig,
) -> Result<MetricsResponse<T>> {
    for attempt in 0..=retry_config.max_retries {
        let response = http
            .get(url)
            .header("Authorization", api_key)
            .send()
            .await?;

        let status = response.status();
        if status.as_u16() == 429 && attempt < retry_config.max_retries {
            let backoff = retry_config.initial_backoff_ms * 2u64.pow(attempt);
            tokio::time::sleep(Duration::from_millis(backoff)).await;
            continue;
        }

        if !status.is_success() {
            let message = response.text().await.unwrap_or_default();
            if status.as_u16() == 429 {
                return Err(ParclError::RateLimited {
                    attempts: attempt + 1,
                    message,
                });
            }
            return Err(ParclError::ApiError {
                status: status.as_u16(),
                message,
            });
        }

        let data: MetricsResponse<T> = response.json().await?;
        return Ok(data);
    }

    unreachable!()
}

/// GETs the initial page, then auto-paginates if enabled.
pub(crate) async fn get_with_pagination<T: DeserializeOwned>(
    http: &Client,
    api_key: &str,
    url: &str,
    auto_paginate: bool,
    retry_config: &RetryConfig,
) -> Result<MetricsResponse<T>> {
    let mut response = get_page(http, api_key, url, retry_config).await?;

    if auto_paginate {
        while let Some(ref next_url) = response.links.next {
            let next_page: MetricsResponse<T> =
                get_page(http, api_key, next_url, retry_config).await?;
            response.items.extend(next_page.items);
            response.links = next_page.links;
        }
    }

    Ok(response)
}

/// Executes a single POST request with retry on 429.
pub(crate) async fn post_page<T: DeserializeOwned>(
    http: &Client,
    api_key: &str,
    url: &str,
    body: &serde_json::Value,
    retry_config: &RetryConfig,
) -> Result<BatchMetricsResponse<T>> {
    for attempt in 0..=retry_config.max_retries {
        let response = http
            .post(url)
            .header("Authorization", api_key)
            .json(body)
            .send()
            .await?;

        let status = response.status();
        if status.as_u16() == 429 && attempt < retry_config.max_retries {
            let backoff = retry_config.initial_backoff_ms * 2u64.pow(attempt);
            tokio::time::sleep(Duration::from_millis(backoff)).await;
            continue;
        }

        if !status.is_success() {
            let message = response.text().await.unwrap_or_default();
            if status.as_u16() == 429 {
                return Err(ParclError::RateLimited {
                    attempts: attempt + 1,
                    message,
                });
            }
            return Err(ParclError::ApiError {
                status: status.as_u16(),
                message,
            });
        }

        let data: BatchMetricsResponse<T> = response.json().await?;
        return Ok(data);
    }

    unreachable!()
}

/// Fetches a follow-up page via GET for batch pagination links (returns BatchMetricsResponse).
async fn batch_get_page<T: DeserializeOwned>(
    http: &Client,
    api_key: &str,
    url: &str,
    retry_config: &RetryConfig,
) -> Result<BatchMetricsResponse<T>> {
    for attempt in 0..=retry_config.max_retries {
        let response = http
            .get(url)
            .header("Authorization", api_key)
            .send()
            .await?;

        let status = response.status();
        if status.as_u16() == 429 && attempt < retry_config.max_retries {
            let backoff = retry_config.initial_backoff_ms * 2u64.pow(attempt);
            tokio::time::sleep(Duration::from_millis(backoff)).await;
            continue;
        }

        if !status.is_success() {
            let message = response.text().await.unwrap_or_default();
            if status.as_u16() == 429 {
                return Err(ParclError::RateLimited {
                    attempts: attempt + 1,
                    message,
                });
            }
            return Err(ParclError::ApiError {
                status: status.as_u16(),
                message,
            });
        }

        let data: BatchMetricsResponse<T> = response.json().await?;
        return Ok(data);
    }

    unreachable!()
}

/// POSTs the initial request, then auto-paginates via GET if enabled.
pub(crate) async fn post_with_pagination<T: DeserializeOwned>(
    http: &Client,
    api_key: &str,
    url: &str,
    body: &serde_json::Value,
    auto_paginate: bool,
    retry_config: &RetryConfig,
) -> Result<BatchMetricsResponse<T>> {
    let mut response = post_page(http, api_key, url, body, retry_config).await?;

    if auto_paginate {
        while let Some(ref next_url) = response.links.next {
            let next_page: BatchMetricsResponse<T> =
                batch_get_page(http, api_key, next_url, retry_config).await?;
            response.items.extend(next_page.items);
            response.links = next_page.links;
        }
    }

    Ok(response)
}
