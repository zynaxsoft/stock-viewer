mod style;
mod ui;

use anyhow::Result;

use iced::{Application, Settings};

#[tokio::main]
async fn main() -> Result<()> {
    sv_core::util::setup_logger()?;
    ui::App::run(Settings {
        default_font: Some(include_bytes!("../../NotoSerifJP-Medium.otf")),
        ..Settings::default()
    }).unwrap();
    Ok(())
}
