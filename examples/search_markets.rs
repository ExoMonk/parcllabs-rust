//! Search for housing markets by name or location.
//!
//! Usage: cargo run --example search_markets

use parcllabs::{LocationType, ParclClient, SearchParams, SortBy, SortOrder, USRegion};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = ParclClient::new()?;

    // Search for Los Angeles markets
    println!("Searching for 'Los Angeles'...\n");
    let params = SearchParams::new().query("Los Angeles").limit(10);
    let results = client.search().markets(params).await?;

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

    // Search for cities in California with sorting
    println!("\n\nSearching for 'San' in California (cities, sorted by population)...\n");
    let params = SearchParams::new()
        .query("San")
        .state("CA")
        .location_type(LocationType::City)
        .sort_by(SortBy::TotalPopulation)
        .sort_order(SortOrder::Desc)
        .limit(5);

    let ca_cities = client.search().markets(params).await?;

    println!("Top California cities matching 'San' by population:");
    for market in &ca_cities.items {
        println!(
            "  {} - pop: {} (parcl_id: {})",
            market.name,
            market.total_population.unwrap_or(0),
            market.parcl_id
        );
    }

    // Search by region
    println!("\n\nSearching for markets in Pacific region...\n");
    let params = SearchParams::new()
        .query("port")
        .region(USRegion::Pacific)
        .limit(5);

    let pacific = client.search().markets(params).await?;

    println!("Pacific region markets matching 'port':");
    for market in &pacific.items {
        println!(
            "  {} ({}) - {}",
            market.name,
            market.location_type,
            market.state_abbreviation.as_deref().unwrap_or("N/A")
        );
    }

    Ok(())
}
