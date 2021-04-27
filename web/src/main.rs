use std::sync::Arc;

use anyhow::Result;

use actix_files::Files;
use actix_web::{web, App, HttpServer, Responder};

mod server;

use sv_core::{
    config::Config,
    extractor::StockResult,
    util,
};

struct AppState {
    config: Arc<Config>,
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let config = &data.config;
    let mut stock_results: Vec<StockResult> = Vec::new();
    let mut tasks = Vec::new();
    for c in &config.stocks {
        let query = util::Query {
            model: c.name.to_string(),
            url: c.sites[0].url.clone(),
        };
        tasks.push(tokio::spawn(async move {
            util::get_stock_result(query).await.unwrap()
        }));
    }
    for task in tasks {
        stock_results.push(task.await.unwrap());
    }
    server::get_index_html_response(&stock_results)
}

#[actix_rt::main]
async fn main() -> Result<()> {
    util::setup_logger()?;
    let config = Arc::new(util::get_config());
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
