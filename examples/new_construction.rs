//! Fetch new construction metrics for a market.
//!
//! Usage: cargo run --example new_construction

use parcllabs::{NewConstructionMetricsParams, ParclClient, PropertyType, SearchParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = ParclClient::new()?;

    // Find Austin, TX — a hot market for new builds
    let params = SearchParams::new().query("Austin").state("TX").limit(1);
    let markets = client.search().markets(params).await?;
    let market = markets.items.first().ok_or("Austin not found")?;

    println!("Market: {} (parcl_id: {})\n", market.name, market.parcl_id);

    // --- New construction event counts (last 3 months) ---
    let params = NewConstructionMetricsParams::new()
        .limit(3)
        .property_type(PropertyType::SingleFamily);

    let counts = client
        .new_construction_metrics()
        .housing_event_counts(market.parcl_id, Some(params))
        .await?;

    println!("New Construction Event Counts (Single Family):");
    println!("{:<12} {:>10} {:>15}", "Date", "Sales", "New Listings");
    println!("{}", "-".repeat(40));

    for item in &counts.items {
        println!(
            "{:<12} {:>10} {:>15}",
            item.date,
            item.sales.unwrap_or(0),
            item.new_listings_for_sale.unwrap_or(0),
        );
    }

    // --- New construction prices (last 3 months) ---
    let params = NewConstructionMetricsParams::new()
        .limit(3)
        .property_type(PropertyType::SingleFamily);

    let prices = client
        .new_construction_metrics()
        .housing_event_prices(market.parcl_id, Some(params))
        .await?;

    println!("\nNew Construction Median Prices (Single Family):");
    println!("{:<12} {:>15} {:>15}", "Date", "Sale Price", "List Price");
    println!("{}", "-".repeat(45));

    for item in &prices.items {
        let sale = item
            .price
            .as_ref()
            .and_then(|p| p.median.as_ref())
            .and_then(|m| m.sales);
        let list = item
            .price
            .as_ref()
            .and_then(|p| p.median.as_ref())
            .and_then(|m| m.new_listings_for_sale);

        println!(
            "{:<12} {:>15} {:>15}",
            item.date,
            sale.map(|v| format!("${:.0}", v))
                .unwrap_or_else(|| "N/A".into()),
            list.map(|v| format!("${:.0}", v))
                .unwrap_or_else(|| "N/A".into()),
        );
    }

    Ok(())
}
