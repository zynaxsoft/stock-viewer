mod style;
mod ui;

use anyhow::Result;

use iced::Application;

#[tokio::main]
async fn main() -> Result<()> {
    sv_core::util::setup_logger()?;
    ui::App::run(iced::Settings::default()).unwrap();
    Ok(())
}
