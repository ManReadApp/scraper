use serde::Deserialize;

pub mod parser;

#[derive(Deserialize)]
pub struct SearchServiceScrapeData {
    url_empty: Option<String>,
    url: String,
    selector: String,
    label_selector: Option<String>,
}
