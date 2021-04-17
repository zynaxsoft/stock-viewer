use scraper::{Html, Selector};


pub struct StockResult {
    pub product_name: String,
    pub price: usize,
}

pub trait ToStockResults {
    fn to_stock_results(&self, body: String) -> Vec<StockResult>;
}

pub struct KakakuExtractor;

impl ToStockResults for KakakuExtractor {
    fn to_stock_results(&self, body: String) -> Vec<StockResult> {
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
}

