use askama::Template;

use actix_web::HttpResponse;

use sv_core::extractor::StockResult;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    index_data: &'a Vec<StockResult>,
}

pub fn get_index_html_response(stock_results: &Vec<StockResult>) -> HttpResponse {
    let body = { IndexTemplate { index_data: &stock_results }.render().unwrap() };
    HttpResponse::Ok().content_type("text/html").body(body)
}
