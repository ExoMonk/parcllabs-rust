//! Batch POST example: fetch metrics for multiple markets in a single request.
//!
//! Usage: cargo run --example batch_metrics

use parcllabs::{MetricsParams, ParclClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = ParclClient::new()?;

    // Two parcl_ids: Los Angeles (2900187) and New York City (2900078)
    let ids = vec![2900187, 2900078];

    let params = MetricsParams::new().limit(2);

    println!("=== Batch Housing Event Counts (2 markets, limit=2) ===\n");

    let resp = client
        .market_metrics()
        .batch_housing_event_counts(ids, Some(params))
        .await?;

    println!(
        "Total: {}, returned: {} (limit: {}, offset: {})\n",
        resp.total,
        resp.items.len(),
        resp.limit,
        resp.offset,
    );
    for item in resp.items.iter().take(6) {
        println!(
            "  parcl_id: {:?}, date: {}, sales: {:?}",
            item.parcl_id, item.date, item.sales,
        );
    }

    Ok(())
}
