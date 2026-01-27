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

        if market.has_price_feed() {
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

    // Demonstrate auto-pagination
    println!("\n\nAuto-pagination demo: Fetching all 'San' cities in California...\n");

    // First, fetch just one page to see the total
    let params = SearchParams::new()
        .query("San")
        .state("CA")
        .location_type(LocationType::City)
        .limit(5);

    let first_page = client.search().markets(params).await?;
    println!(
        "Without auto_paginate: fetched {} of {} total results\n",
        first_page.items.len(),
        first_page.total
    );

    // Now fetch all pages
    let params = SearchParams::new()
        .query("San")
        .state("CA")
        .location_type(LocationType::City)
        .limit(5)
        .auto_paginate(true);

    let all_results = client.search().markets(params).await?;
    println!(
        "With auto_paginate: fetched {} of {} total results:",
        all_results.items.len(),
        all_results.total
    );
    for (i, market) in all_results.items.iter().enumerate() {
        println!(
            "  {}. {} (parcl_id: {})",
            i + 1,
            market.name,
            market.parcl_id
        );
    }

    Ok(())
}
