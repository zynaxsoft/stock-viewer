use std::sync::Arc;

mod util;
mod server;
mod config;

use util::*;
use config::Config;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = Config::from_file("./config.toml");
    let config = Arc::new(config);
    let query = Query{url: config.stocks[0].url.clone()};
    let body = get_html_text(&query).await?;
    parse_html(body, &query);
    // println!("{}", body);
    Ok(())
}
