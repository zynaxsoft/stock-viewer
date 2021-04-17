use std::sync::Arc;

mod util;
mod server;
mod config;
mod extractor;

use util::*;
use config::Config;
use extractor::ToStockResults;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = Config::from_file("./config.toml");
    let config = Arc::new(config);
    let query = Query{url: config.stocks[0].sites[0].url.clone()};
    let body = get_html_text(&query).await?;
    extractor::KakakuExtractor.to_stock_results(body);
    // println!("{}", body);
    Ok(())
}
