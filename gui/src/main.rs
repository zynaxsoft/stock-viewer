mod style;
mod ui;

use anyhow::Result;

use iced::Application;

use sv_core::{extractor::StockResult, util};

#[tokio::main]
async fn main() -> Result<()> {
    util::setup_logger()?;
    ui::App::run(iced::Settings::default()).unwrap();
    Ok(())
}
