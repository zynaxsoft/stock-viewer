use std::fmt;

#[derive(Debug)]
pub enum MyError {
    NoGood,
}
impl std::error::Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self)
    }
}

pub struct Query {
    pub url: String,
}

pub async fn get_html_text(query: &Query) -> Result<String, reqwest::Error> {
    let body = reqwest::get(&query.url).await?.text().await?;
    Ok(body)
}

