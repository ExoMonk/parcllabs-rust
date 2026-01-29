//! Analyze investor buying/selling activity to identify market trends.
//!
//! This example demonstrates how to:
//! - Track investor purchase-to-sale ratios (are investors net buyers or sellers?)
//! - Monitor investor transaction volumes
//! - Compare investor activity across major metros
//! - Identify markets where institutional money is flowing in or out
//!
//! Use Case: Real estate analysts tracking where institutional investors
//! are accumulating or liquidating positions.
//!
//! Usage: cargo run --example investor_activity

use parcllabs::{InvestorMetricsParams, ParclClient, SearchParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = ParclClient::new()?;

    println!("=======================================================");
    println!("  INVESTOR ACTIVITY ANALYSIS - Major U.S. Metros");
    println!("=======================================================\n");

    // Define major metros to analyze
    let metros = vec![
        ("Phoenix", "AZ"),
        ("Atlanta", "GA"),
        ("Dallas", "TX"),
        ("Tampa", "FL"),
        ("Charlotte", "NC"),
        ("Las Vegas", "NV"),
    ];

    // Collect data for each metro
    let mut market_data = Vec::new();

    for (city, state) in &metros {
        let params = SearchParams::new().query(*city).state(*state).limit(1);

        let markets = client.search().markets(params).await?;

        if let Some(market) = markets.items.first() {
            // Get purchase-to-sale ratio (last 6 months)
            let ratio_params = InvestorMetricsParams::new().limit(6);

            let ratios = client
                .investor_metrics()
                .purchase_to_sale_ratio(market.parcl_id, Some(ratio_params))
                .await?;

            // Get investor event counts
            let counts = client
                .investor_metrics()
                .housing_event_counts(market.parcl_id, Some(InvestorMetricsParams::new().limit(1)))
                .await?;

            if let (Some(latest_ratio), Some(latest_counts)) =
                (ratios.items.first(), counts.items.first())
            {
                let ratio = latest_ratio.purchase_to_sale_ratio.unwrap_or(0.0);
                let acquisitions = latest_counts.acquisitions.unwrap_or(0);
                let dispositions = latest_counts.dispositions.unwrap_or(0);

                // Calculate average ratio over period
                let avg_ratio: f64 = ratios
                    .items
                    .iter()
                    .filter_map(|r| r.purchase_to_sale_ratio)
                    .sum::<f64>()
                    / ratios.items.len().max(1) as f64;

                market_data.push((
                    format!("{}, {}", city, state),
                    market.parcl_id,
                    ratio,
                    avg_ratio,
                    acquisitions,
                    dispositions,
                    latest_ratio.date.clone(),
                ));
            }
        }
    }

    // Sort by current ratio (highest first = strongest buyer activity)
    market_data.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    // Display results
    println!("INVESTOR PURCHASE-TO-SALE RATIO BY METRO");
    println!("(Ratio > 1.0 = Net Buyers, < 1.0 = Net Sellers)\n");
    println!(
        "{:<20} {:>12} {:>12} {:>12} {:>12}",
        "Metro", "Current", "6-Mo Avg", "Buys", "Sells"
    );
    println!("{}", "-".repeat(70));

    for (name, _parcl_id, ratio, avg_ratio, acquisitions, dispositions, _date) in &market_data {
        let trend = if *ratio > *avg_ratio {
            "↑"
        } else if *ratio < *avg_ratio {
            "↓"
        } else {
            "→"
        };

        let status = if *ratio > 1.2 {
            "🔥 Strong Buy"
        } else if *ratio > 1.0 {
            "📈 Net Buyer"
        } else if *ratio > 0.8 {
            "📉 Net Seller"
        } else {
            "❄️  Exiting"
        };

        println!(
            "{:<20} {:>10.2} {} {:>10.2} {:>12} {:>12}",
            name, ratio, trend, avg_ratio, acquisitions, dispositions
        );
        println!("{:<20} {}", "", status);
    }

    // Deep dive into top market
    if let Some((top_name, top_id, ..)) = market_data.first() {
        println!("\n=======================================================");
        println!("  DEEP DIVE: {} (parcl_id: {})", top_name, top_id);
        println!("=======================================================\n");

        // Get rolling counts for new listings
        let rolling = client
            .investor_metrics()
            .new_listings_for_sale_rolling_counts(
                *top_id,
                Some(InvestorMetricsParams::new().limit(4)),
            )
            .await?;

        println!("INVESTOR NEW LISTINGS ACTIVITY (Rolling Counts):\n");
        println!(
            "{:<12} {:>10} {:>10} {:>10} {:>10}",
            "Date", "7-Day", "30-Day", "60-Day", "90-Day"
        );
        println!("{}", "-".repeat(55));

        for item in &rolling.items {
            let counts = item.count.as_ref();
            println!(
                "{:<12} {:>10} {:>10} {:>10} {:>10}",
                item.date,
                counts.and_then(|c| c.rolling_7_day).unwrap_or(0),
                counts.and_then(|c| c.rolling_30_day).unwrap_or(0),
                counts.and_then(|c| c.rolling_60_day).unwrap_or(0),
                counts.and_then(|c| c.rolling_90_day).unwrap_or(0)
            );
        }

        // Get investor market share
        println!("\nINVESTOR MARKET SHARE (% of new listings):\n");
        println!(
            "{:<12} {:>10} {:>10} {:>10} {:>10}",
            "Date", "7-Day %", "30-Day %", "60-Day %", "90-Day %"
        );
        println!("{}", "-".repeat(55));

        for item in &rolling.items {
            let pcts = item.pct_for_sale_market.as_ref();
            println!(
                "{:<12} {:>9.1}% {:>9.1}% {:>9.1}% {:>9.1}%",
                item.date,
                pcts.and_then(|p| p.rolling_7_day).unwrap_or(0.0),
                pcts.and_then(|p| p.rolling_30_day).unwrap_or(0.0),
                pcts.and_then(|p| p.rolling_60_day).unwrap_or(0.0),
                pcts.and_then(|p| p.rolling_90_day).unwrap_or(0.0)
            );
        }

        // Historical trend analysis
        let historical = client
            .investor_metrics()
            .purchase_to_sale_ratio(
                *top_id,
                Some(InvestorMetricsParams::new().limit(12).auto_paginate(true)),
            )
            .await?;

        println!("\n12-MONTH PURCHASE-TO-SALE RATIO TREND:\n");
        println!("{:<12} {:>10}  Visual", "Date", "Ratio");
        println!("{}", "-".repeat(50));

        for item in historical.items.iter().rev() {
            let ratio = item.purchase_to_sale_ratio.unwrap_or(0.0);
            let bar_len = (ratio * 20.0).min(40.0) as usize;
            let bar = "█".repeat(bar_len);

            let marker = if ratio > 1.0 { ">" } else { "<" };

            println!("{:<12} {:>10.2}  |{}{}", item.date, ratio, bar, marker);
        }
        println!("{:<12} {:>10}  |{}", "", "1.0", "-".repeat(20));
        println!("{:<24}   >1.0 = Net Buyer, <1.0 = Net Seller", "");
    }

    println!("\n=======================================================");
    println!("  ANALYSIS COMPLETE");
    println!("=======================================================");

    Ok(())
}
