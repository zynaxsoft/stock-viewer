pub struct Query {
    pub url: String,
}

pub async fn get_html_text(query: &Query) -> Result<String, reqwest::Error> {
    let body = reqwest::get(&query.url).await?.text().await?;
    Ok(body)
}

