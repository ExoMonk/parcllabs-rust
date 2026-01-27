//! Fetch housing metrics for a market.
//!
//! Usage: cargo run --example market_metrics

use parcllabs::{MetricsParams, ParclClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = ParclClient::new()?;

    // First, find Los Angeles
    let markets = client
        .search()
        .markets("Los Angeles", Some("CA"), None, Some(1))
        .await?;

    let la = markets.items.first().ok_or("Los Angeles not found")?;

    println!("Market: {} (parcl_id: {})\n", la.name, la.parcl_id);

    // Get recent housing event counts
    let params = MetricsParams::new().limit(6).start_date("2024-01-01");

    let events = client
        .market_metrics()
        .housing_event_counts(la.parcl_id, Some(params))
        .await?;

    println!("Housing Events (2024):");
    println!("{:<12} {:>10} {:>15}", "Date", "Sales", "New Listings");
    println!("{}", "-".repeat(40));

    for event in &events.items {
        println!(
            "{:<12} {:>10} {:>15}",
            event.date,
            event.sales.unwrap_or(0),
            event.new_listings_for_sale.unwrap_or(0)
        );
    }

    // Get housing stock
    let stock = client
        .market_metrics()
        .housing_stock(la.parcl_id, Some(MetricsParams::new().limit(1)))
        .await?;

    if let Some(latest) = stock.items.first() {
        println!("\nHousing Stock ({}):", latest.date);
        println!("  Single Family: {:>10}", latest.single_family.unwrap_or(0));
        println!("  Condo:         {:>10}", latest.condo.unwrap_or(0));
        println!("  Townhouse:     {:>10}", latest.townhouse.unwrap_or(0));
        println!("  Total:         {:>10}", latest.total.unwrap_or(0));
    }

    // Get prices
    let prices = client
        .market_metrics()
        .housing_event_prices(la.parcl_id, Some(MetricsParams::new().limit(3)))
        .await?;

    println!("\nRecent Prices:");
    for price in &prices.items {
        println!(
            "  {}: Sale ${:.0}k, List ${:.0}k, Rent ${:.0}/mo",
            price.date,
            price.median_sale_price.unwrap_or(0.0) / 1000.0,
            price.median_list_price.unwrap_or(0.0) / 1000.0,
            price.median_rental_price.unwrap_or(0.0)
        );
    }

    Ok(())
}
