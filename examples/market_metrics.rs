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
        println!("  Other:         {:>10}", latest.other.unwrap_or(0));
        println!(
            "  All Properties:{:>10}",
            latest.all_properties.unwrap_or(0)
        );
    }

    // Get prices
    let prices = client
        .market_metrics()
        .housing_event_prices(la.parcl_id, Some(MetricsParams::new().limit(3)))
        .await?;

    println!("\nRecent Median Prices:");
    for item in &prices.items {
        if let Some(ref price) = item.price {
            if let Some(ref median) = price.median {
                println!(
                    "  {}: Sale ${:.0}k, List ${:.0}k, Rent ${:.0}/mo",
                    item.date,
                    median.sales.unwrap_or(0.0) / 1000.0,
                    median.new_listings_for_sale.unwrap_or(0.0) / 1000.0,
                    median.new_rental_listings.unwrap_or(0.0)
                );
            }
        }
    }

    Ok(())
}
