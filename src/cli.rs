use clap::Parser;
use reqwest::Url;

#[derive(Parser, Debug, Clone)]
#[clap()]
pub struct Cli {
    #[arg(short, long)]
    pub router_base_api_url: String,
    /// Limit the number of pairs to check
    #[arg(short, long)]
    pub limit: Option<usize>,
}


