//! Search for housing markets by name or location.
//!
//! Usage: PARCL_LABS_API_KEY=your_key cargo run --example search_markets

use parcllabs::{LocationType, ParclClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // Search by state and location type
    println!("\n\nSearching for cities in California...\n");
    let ca_cities = client
        .search()
        .markets_by_state("CA", Some(LocationType::City), Some(5))
        .await?;

    println!("Top {} California cities:", ca_cities.items.len());
    for market in &ca_cities.items {
        println!("  {} (parcl_id: {})", market.name, market.parcl_id);
    }

    Ok(())
}
