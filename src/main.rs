use std::sync::Arc;

use anyhow::Result;

use actix_files::Files;
use actix_web::{web, App, HttpServer, Responder};

mod config;
mod extractor;
mod server;
mod util;

use config::Config;
use extractor::ToStockResult;
use util::*;

struct AppState {
    config: Arc<Config>,
}

fn get_config() -> Arc<Config> {
    let config: Config = Config::from_file("./config.toml");
    log::info!("config.toml loaded.");
    Arc::new(config)
}

async fn get_stock_result(query: Query) -> Result<extractor::StockResult> {
    let stock_html = get_html_text(&query).await?;
    log::info!("Successfully acquired html bodies.");
    Ok(extractor::KakakuExtractor.to_stock_result(query.model, stock_html))
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let config = &data.config;
    let mut stock_results = Vec::new();
    let mut tasks = Vec::new();
    for c in &config.stocks {
        let query = Query {
            model: c.name.to_string(),
            url: c.sites[0].url.clone(),
        };
        tasks.push(tokio::spawn(async move {
            get_stock_result(query).await.unwrap()
        }));
    }
    for task in tasks {
        stock_results.push(task.await.unwrap());
    }
    server::get_index_html_response(&stock_results)
}

#[actix_rt::main]
async fn main() -> Result<()> {
    setup_logger()?;
    let config = get_config();
    let address = format!("{}:{}", config.server.ip, config.server.port);
    log::info!("Serving on {}", address);
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                config: Arc::clone(&config),
            })
            .route("/", web::get().to(index))
            .service(Files::new("/images", "./images").show_files_listing())
    })
    .bind(address)?
    .run()
    .await?;
    Ok(())
}
