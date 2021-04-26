mod style;
mod ui;

use anyhow::Result;

use iced::Sandbox;

#[tokio::main]
async fn main() -> Result<()> {
    ui::App::run(iced::Settings::default()).unwrap();
    Ok(())
}
