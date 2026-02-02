//! Fetch portfolio metrics segmented by portfolio size.
//!
//! Usage: cargo run --example portfolio_metrics

use parcllabs::{ParclClient, PortfolioMetricsParams, PortfolioSize, SearchParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = ParclClient::new()?;

    // Find Atlanta, GA — strong institutional investor market
    let params = SearchParams::new().query("Atlanta").state("GA").limit(1);
    let markets = client.search().markets(params).await?;
    let market = markets.items.first().ok_or("Atlanta not found")?;

    println!("Market: {} (parcl_id: {})\n", market.name, market.parcl_id);

    // --- SF housing stock ownership (broken down by portfolio size) ---
    let params = PortfolioMetricsParams::new().limit(1);
    let ownership = client
        .portfolio_metrics()
        .sf_housing_stock_ownership(market.parcl_id, Some(params))
        .await?;

    if let Some(item) = ownership.items.first() {
        println!("SF Housing Stock Ownership ({}):", item.date);
        println!("{:<20} {:>10} {:>8}", "Portfolio Size", "Count", "Pct");
        println!("{}", "-".repeat(40));

        if let (Some(c), Some(p)) = (&item.count, &item.pct_sf_housing_stock) {
            println!(
                "{:<20} {:>10} {:>7.1}%",
                "2-9 units",
                c.portfolio_2_to_9.unwrap_or(0),
                p.portfolio_2_to_9.unwrap_or(0.0)
            );
            println!(
                "{:<20} {:>10} {:>7.1}%",
                "10-99 units",
                c.portfolio_10_to_99.unwrap_or(0),
                p.portfolio_10_to_99.unwrap_or(0.0)
            );
            println!(
                "{:<20} {:>10} {:>7.1}%",
                "100-999 units",
                c.portfolio_100_to_999.unwrap_or(0),
                p.portfolio_100_to_999.unwrap_or(0.0)
            );
            println!(
                "{:<20} {:>10} {:>7.1}%",
                "1000+ units",
                c.portfolio_1000_plus.unwrap_or(0),
                p.portfolio_1000_plus.unwrap_or(0.0)
            );
            println!("{}", "-".repeat(40));
            println!(
                "{:<20} {:>10} {:>7.1}%",
                "All Portfolios",
                c.all_portfolios.unwrap_or(0),
                p.all_portfolios.unwrap_or(0.0)
            );
        }
    }

    // --- SF event counts (large portfolios: 1000+) ---
    let params = PortfolioMetricsParams::new()
        .limit(3)
        .portfolio_size(PortfolioSize::Portfolio1000Plus);

    let counts = client
        .portfolio_metrics()
        .sf_housing_event_counts(market.parcl_id, Some(params))
        .await?;

    println!("\nSF Event Counts (Portfolio 1000+):");
    println!(
        "{:<12} {:>8} {:>8} {:>12} {:>12}",
        "Date", "Acq", "Disp", "For Sale", "Rentals"
    );
    println!("{}", "-".repeat(55));

    for item in &counts.items {
        println!(
            "{:<12} {:>8} {:>8} {:>12} {:>12}",
            item.date,
            item.acquisitions.unwrap_or(0),
            item.dispositions.unwrap_or(0),
            item.new_listings_for_sale.unwrap_or(0),
            item.new_rental_listings.unwrap_or(0),
        );
    }

    // --- Rolling for-sale listing counts (all portfolios) ---
    let params = PortfolioMetricsParams::new()
        .limit(3)
        .portfolio_size(PortfolioSize::AllPortfolios);

    let rolling = client
        .portfolio_metrics()
        .sf_new_listings_for_sale_rolling_counts(market.parcl_id, Some(params))
        .await?;

    println!("\nSF New For-Sale Listings Rolling Counts (All Portfolios):");
    println!(
        "{:<12} {:>8} {:>8} {:>8} {:>8}  {:>6} {:>6} {:>6} {:>6}",
        "Date", "7d", "30d", "60d", "90d", "7d%", "30d%", "60d%", "90d%"
    );
    println!("{}", "-".repeat(80));

    for item in &rolling.items {
        let c = item.count.as_ref();
        let p = item.pct_sf_for_sale_market.as_ref();
        println!(
            "{:<12} {:>8} {:>8} {:>8} {:>8}  {:>5.1}% {:>5.1}% {:>5.1}% {:>5.1}%",
            item.date,
            c.and_then(|c| c.rolling_7_day).unwrap_or(0),
            c.and_then(|c| c.rolling_30_day).unwrap_or(0),
            c.and_then(|c| c.rolling_60_day).unwrap_or(0),
            c.and_then(|c| c.rolling_90_day).unwrap_or(0),
            p.and_then(|p| p.rolling_7_day).unwrap_or(0.0),
            p.and_then(|p| p.rolling_30_day).unwrap_or(0.0),
            p.and_then(|p| p.rolling_60_day).unwrap_or(0.0),
            p.and_then(|p| p.rolling_90_day).unwrap_or(0.0),
        );
    }

    Ok(())
}
