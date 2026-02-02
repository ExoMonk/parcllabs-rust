//! Shared POST helpers for batch endpoints.

use crate::error::{ParclError, Result};
use crate::models::BatchMetricsResponse;
use reqwest::Client;
use serde::de::DeserializeOwned;

/// Executes a single POST request and deserializes the response.
pub(crate) async fn post_page<T: DeserializeOwned>(
    http: &Client,
    api_key: &str,
    url: &str,
    body: &serde_json::Value,
) -> Result<BatchMetricsResponse<T>> {
    let response = http
        .post(url)
        .header("Authorization", api_key)
        .json(body)
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let message = response.text().await.unwrap_or_default();
        return Err(ParclError::ApiError {
            status: status.as_u16(),
            message,
        });
    }

    let data: BatchMetricsResponse<T> = response.json().await?;
    Ok(data)
}

/// Fetches a follow-up page via GET (for pagination links).
async fn get_page<T: DeserializeOwned>(
    http: &Client,
    api_key: &str,
    url: &str,
) -> Result<BatchMetricsResponse<T>> {
    let response = http
        .get(url)
        .header("Authorization", api_key)
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let message = response.text().await.unwrap_or_default();
        return Err(ParclError::ApiError {
            status: status.as_u16(),
            message,
        });
    }

    let data: BatchMetricsResponse<T> = response.json().await?;
    Ok(data)
}

/// POSTs the initial request, then auto-paginates via GET if enabled.
pub(crate) async fn post_with_pagination<T: DeserializeOwned>(
    http: &Client,
    api_key: &str,
    url: &str,
    body: &serde_json::Value,
    auto_paginate: bool,
) -> Result<BatchMetricsResponse<T>> {
    let mut response = post_page(http, api_key, url, body).await?;

    if auto_paginate {
        while let Some(ref next_url) = response.links.next {
            let next_page: BatchMetricsResponse<T> = get_page(http, api_key, next_url).await?;
            response.items.extend(next_page.items);
            response.links = next_page.links;
        }
    }

    Ok(response)
}
