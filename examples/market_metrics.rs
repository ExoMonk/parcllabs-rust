//! Fetch housing metrics for a market.
//!
//! Usage: cargo run --example market_metrics

use parcllabs::{MetricsParams, ParclClient, PropertyType, SearchParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = ParclClient::new()?;

    // First, find Los Angeles
    let params = SearchParams::new()
        .query("Los Angeles")
        .state("CA")
        .limit(1);
    let markets = client.search().markets(params).await?;

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

    // Get prices (all properties)
    let prices = client
        .market_metrics()
        .housing_event_prices(la.parcl_id, Some(MetricsParams::new().limit(1)))
        .await?;

    println!("\nRecent Median Prices (All Properties):");
    if let Some(item) = prices.items.first() {
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

    // Get prices filtered by property type
    let sf_params = MetricsParams::new()
        .limit(1)
        .property_type(PropertyType::SingleFamily);

    let sf_prices = client
        .market_metrics()
        .housing_event_prices(la.parcl_id, Some(sf_params))
        .await?;

    println!("\nRecent Median Prices (Single Family Only):");
    if let Some(item) = sf_prices.items.first() {
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

    // Compare with Condo prices
    let condo_params = MetricsParams::new()
        .limit(1)
        .property_type(PropertyType::Condo);

    let condo_prices = client
        .market_metrics()
        .housing_event_prices(la.parcl_id, Some(condo_params))
        .await?;

    println!("\nRecent Median Prices (Condo Only):");
    if let Some(item) = condo_prices.items.first() {
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

    // Demonstrate auto-pagination for metrics
    println!("\n--- Auto-pagination Demo ---");

    // First, fetch one page to see the total
    let params = MetricsParams::new().limit(5).start_date("2024-01-01");
    let first_page = client
        .market_metrics()
        .housing_event_counts(la.parcl_id, Some(params))
        .await?;
    println!(
        "\nWithout auto_paginate: fetched {} of {} total events",
        first_page.items.len(),
        first_page.total
    );

    // Now fetch all pages
    let params = MetricsParams::new()
        .limit(5)
        .start_date("2024-01-01")
        .auto_paginate(true);
    let all_events = client
        .market_metrics()
        .housing_event_counts(la.parcl_id, Some(params))
        .await?;
    println!(
        "With auto_paginate: fetched {} of {} total events",
        all_events.items.len(),
        all_events.total
    );

    Ok(())
}
