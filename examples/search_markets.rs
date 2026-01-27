//! Search for housing markets by name or location.
//!
//! Usage: cargo run --example search_markets

use parcllabs::{LocationType, ParclClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = ParclClient::new()?;

    // Search for Los Angeles markets
    println!("Searching for 'Los Angeles'...\n");
    let results = client
        .search()
        .markets("Los Angeles", None, None, Some(10))
        .await?;

    println!(
        "Found {} markets (showing first {}):\n",
        results.total,
        results.items.len()
    );

    for market in &results.items {
        println!(
            "  [{:>10}] {} ({}) - pop: {}, income: ${}",
            market.parcl_id,
            market.name,
            market.location_type,
            market.total_population.unwrap_or(0),
            market.median_income.unwrap_or(0)
        );

        if market.pricefeed_market == Some(1) {
            println!("              ^ Price feed available");
        }
    }

    // Search for cities in California
    println!("\n\nSearching for 'San' in California (cities only)...\n");
    let ca_cities = client
        .search()
        .markets("San", Some("CA"), Some(LocationType::City), Some(5))
        .await?;

    println!("California cities matching 'San':");
    for market in &ca_cities.items {
        println!("  {} (parcl_id: {})", market.name, market.parcl_id);
    }

    Ok(())
}
