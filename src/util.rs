use std::fmt;

use scraper::element_ref::ElementRef;
use scraper::{Html, Selector};


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

pub struct StockResult {
    pub product_name: String,
    pub price: usize,
}

pub async fn get_html_text(query: &Query) -> Result<String, reqwest::Error> {
    let body = reqwest::get(&query.url).await?.text().await?;
    Ok(body)
}

pub fn parse_html(body: String, query: &Query) -> Vec<StockResult> {
    let result = Vec::new();
    let document = Html::parse_document(&body);
    let selector = Selector::parse("li.pryen").unwrap();
    for entry in document.select(&selector) {
        println!("{}", entry.html());
        println!("----------------------");
    }
    let selector = Selector::parse("a.ckitanker").unwrap();
    for entry in document.select(&selector) {
        println!("{}", entry.html());
        println!("----------------------");
    }
    result
}

