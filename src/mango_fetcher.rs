use anyhow::Context;
use serde_derive::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashSet;
use std::str::FromStr;
use std::time::Duration;

#[derive(Clone)]
pub struct MangoMetadata {
    pub mints: HashSet<Pubkey>,
    pub obv2_markets: HashSet<Pubkey>,
}

pub fn read_mints_from_file() -> HashSet<Pubkey>{
    let contents = include_str!("mango-mints-pubkeys.dat");
    contents
        .lines()
        .map(|line| Pubkey::from_str(line).unwrap())
        .collect()
}

/// Return (mints, obv2-markets)
pub async fn fetch_mango_data() -> anyhow::Result<MangoMetadata> {
    let address = "https://api.mngo.cloud/data/v4/group-metadata";
    let http_client = reqwest::Client::new();
    let response = http_client
        .get(address)
        .timeout(Duration::from_secs(60))
        .send()
        .await
        .context("mango group request")?;

    let metadata = serde_json::from_str::<MangoGroupMetadataResponse>(&response.text().await?)?;

    let mut mints = HashSet::new();
    let mut obv2_markets = HashSet::new();

    for group in &metadata.groups {
        for token in &group.tokens {
            mints.insert(Pubkey::from_str(token.mint.as_str())?);
        }
        for market in &group.openbook_v2_markets {
            obv2_markets.insert(Pubkey::from_str(market.serum_market_external.as_str())?);
        }
    }

    Ok(MangoMetadata {
        mints,
        obv2_markets,
    })
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MangoGroupMetadataResponse {
    pub groups: Vec<MangoGroupMetadata>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MangoGroupMetadata {
    pub tokens: Vec<MangoGroupTokenMetadata>,
    pub openbook_v2_markets: Vec<MangoGroupObv2MarketsMetadata>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MangoGroupTokenMetadata {
    pub mint: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MangoGroupObv2MarketsMetadata {
    pub serum_market_external: String,
}
