use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Price {
    pub digit: usize,
    pub string: String,
}

#[derive(Debug)]
pub struct ScrappedData {
    pub name_html: String,
    pub html_link: String,
    pub price: Price,
}

#[derive(Debug)]
pub struct StockResult {
    pub model: String,
    pub data: Vec<ScrappedData>,
}

pub trait ToStockResult {
    fn to_stock_result(&self, model: String, body: String) -> StockResult;
}

pub struct KakakuExtractor;

impl ToStockResult for KakakuExtractor {
    fn to_stock_result(&self, model: String, body: String) -> StockResult {
        let mut result = Vec::new();
        let document = Html::parse_document(&body);
        let name_sel = Selector::parse("a.ckitanker").unwrap();
        let price_sel = Selector::parse("li.pryen").unwrap();
        let name_iter = document.select(&name_sel);
        let price_iter = document.select(&price_sel);
        for (name_entry, price_entry) in name_iter.zip(price_iter) {
            let name_html = name_entry.inner_html();
            let html_link = name_entry.html();
            let price_html = price_entry.inner_html();
            let price_string = price_html
                .splitn(2, '¥')
                .nth(1)
                .unwrap()
                .trim_end_matches("</a>"); // "200,000"
            let price = price_string
                .replace(",", "") // "200000"
                .parse::<usize>()
                .unwrap();
            result.push(ScrappedData {
                name_html,
                html_link,
                price: Price {
                    digit: price,
                    string: price_string.to_string(),
                },
            })
        }
        log::debug!("KakakuExtractor Vec<StockResult>\n{:#?}", result);
        StockResult {
            model,
            data: result,
        }
    }
}
