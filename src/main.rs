use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use clap::Parser;
use serde::__private::ser::CannotSerializeVariant;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use tracing::{debug, error, info, trace, warn};
use url::Url;
use crate::cli::Cli;
use crate::mango_fetcher::read_mints_from_file;
use crate::oha_result::{read_error_text, read_oha_result};

mod mango_fetcher;
mod oha_result;
mod oha_client;
mod cli;

#[tokio::main]
async fn main()
{
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO).init();

    let Cli { router_base_api_url, limit, oha_bin_path } = Cli::parse();
    let limit = limit.unwrap_or(999_999);
    let oha_bin_path = oha_bin_path.unwrap_or("oha".to_string());

    let router_base_api_url = Url::parse(&router_base_api_url).expect("URL parsing failed");
    info!("router_base_api_url: {}", router_base_api_url);
    info!("oha_bin_path: {}", &oha_bin_path);
    info!("oha version string: \"{}\"", oha_client::read_oha_version(&oha_bin_path));
    info!("limit: {}", limit);

    std::fs::create_dir("output").unwrap_or_default();

    // let mango_metadata = mango_fetcher::fetch_mango_data().await.unwrap();
    // let mints = mango_metadata.mints;
    let mints = read_mints_from_file();

    info!("mango listed mints: {}", mints.len());
    for mint_pubkey in &mints {
        debug!("mint: {}", mint_pubkey);
    }
    
    let pairs = build_all_combinations(&mints);
    info!("all pairs: {}", pairs.len());

    for (input_mint, output_mint) in &pairs {
        debug!("input: {}, output: {}", input_mint, output_mint);
    }

    let mut report_success_csv = csv::Writer::from_path("output/report-success.csv").unwrap();
    report_success_csv.write_record(&["input_mint", "output_mint", "status_200", "latency_p99"]).unwrap();
    let mut report_fails_csv = csv::Writer::from_path("output/report-fails.csv").unwrap();
    report_fails_csv.write_record(&["input_mint", "output_mint", "error_text", "error_count"]).unwrap();
    for (input_mint, output_mint) in pairs.iter().take(limit) {
        let result_json = oha_client::call_quote(input_mint, output_mint, &oha_bin_path, router_base_api_url.clone());
        let res = read_oha_result(&result_json);
        let json = &result_json;
        info!("benching with input={}, output={}", input_mint, output_mint);
        match &res {
            Some((status_200, latency_p99)) => {
                info!("> status 200 cnt: {}", status_200);
                info!("> latency p99: {:.2}ms", latency_p99.as_secs_f64() * 1000.0);
            }
            None => {
                let compact = serde_json::to_string(&json).unwrap();
                warn!("> failed to read oha result: {}", compact);
                trace!("formatted result:\n{}", serde_json::to_string_pretty(&json).unwrap());
            }
        }
        if let Some((status_200, latency_p99)) = res {
            report_success_csv.write_record(&[
                input_mint.to_string(),
                output_mint.to_string(),
                format!("{}", status_200),
                format!("{:.2}", latency_p99.as_secs_f64() * 1000.0), // ms
            ]).unwrap()
        } else {
            let error_res: Option<Vec<(String, u64)>> = read_error_text(&result_json);
            if let Some(error_res) = error_res {
                error_res.iter().for_each(|(error_text, count)| {
                    report_fails_csv.write_record(&[
                        input_mint.to_string(),
                        output_mint.to_string(),
                        error_text.to_string(),
                        format!("{}", count),
                    ]).unwrap()
                });
            } else {
                error!("Failed to read error: {}", serde_json::to_string(&result_json).unwrap());
            }
        }
        report_success_csv.flush().unwrap();
        report_fails_csv.flush().unwrap();
    }
    report_success_csv.flush().unwrap();

}

fn build_all_combinations(mints: &HashSet<Pubkey>) -> Vec<(Pubkey, Pubkey)> {
    let mut pairs: Vec<(Pubkey, Pubkey)> = Vec::new();
    let mut mints = mints.clone();
    let mint_sol = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
    let mint_usdc = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
    mints.retain(|mint| mint != &mint_sol || mint != &mint_usdc);

    // foo -> SOL
    for mint in &mints {
        pairs.push((mint.clone(), mint_sol.clone()));
    }

    // foo -> USDC
    for mint in &mints {
        pairs.push((mint.clone(), mint_usdc.clone()));
    }

    // SOL -> foo
    for mint in &mints {
        pairs.push((mint_sol.clone(), mint.clone()));
    }

    // USDC -> foo
    for mint in &mints {
        pairs.push((mint_usdc.clone(), mint.clone()));
    }

    pairs
}
