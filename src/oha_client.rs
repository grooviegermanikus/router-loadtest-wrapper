use std::process::Command;
use reqwest::Url;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

pub fn read_oha_version(oha_bin_path: &str) -> String {
    let output = Command::new(oha_bin_path)
        .args(&["--version"])
        .output()
        .expect("failed to execute 'oha' process");

    String::from_utf8(output.stdout).unwrap().replace("\n", "")
}

pub fn call_quote(inputMint: &Pubkey, outputMint: &Pubkey, oha_bin_path: &str, router_api_base_url: Url) -> Value {
    let router_api_url = build_router_api_url(inputMint, outputMint, router_api_base_url);

    let mut args = Vec::new();
    args.push("--no-tui");
    args.push("--json");
    args.push(router_api_url.as_str());
    args.push("-c2");
    args.push("-z1s");

    let output = Command::new(oha_bin_path)
        .args(&args)
        .output()
        .expect("failed to execute 'oha' process");

    let hello = output.stdout;
    let json = serde_json::from_slice::<serde_json::Value>(&hello).unwrap();

    json
}

fn build_router_api_url(inputMint: &Pubkey, outputMint: &Pubkey, base_url: Url) -> Url {
    let mut router_api_url = base_url;

    router_api_url.set_path("quote");
    {
        let mut query_params = router_api_url.query_pairs_mut();
        query_params.append_pair("inputMint", &inputMint.to_string());
        query_params.append_pair("outputMint", &outputMint.to_string());
        query_params.append_pair("amount", "100000000");
        query_params.append_pair("slippageBps", "50");
    }

    router_api_url
}