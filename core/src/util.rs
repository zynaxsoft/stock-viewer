use fern::colors::{Color, ColoredLevelConfig};

pub struct Query {
    pub model: String,
    pub url: String,
}

pub async fn get_html_text(query: &Query) -> Result<String, reqwest::Error> {
    let body = reqwest::get(&query.url).await?.text().await?;
    Ok(body)
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    let mut colors = ColoredLevelConfig::new().info(Color::Green);
    colors.warn = Color::Magenta;

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Warn)
        .level_for("sv_core", log::LevelFilter::Debug)
        .level_for("web", log::LevelFilter::Debug)
        .level_for("gui", log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
