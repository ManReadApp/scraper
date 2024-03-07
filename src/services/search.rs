use crate::extractor::SearchServiceScrapeData;
use crate::ScrapeError;
use api_structure::scraper::{SimpleSearch, ValidSearch};
use reqwest::Client;
use std::collections::HashMap;

#[derive(Default)]
pub struct SearchService {
    client: Client,
    services: HashMap<String, SearchServiceScrapeData>,
}

impl SearchService {
    pub fn new(services: HashMap<String, SearchServiceScrapeData>) -> Self {
        Self {
            client: Default::default(),
            services,
        }
    }
    pub fn sites() -> Vec<String> {
        todo!()
    }

    pub async fn search(uri: &str, search: SimpleSearch) -> Result<Vec<SearchResult>, ScrapeError> {
        todo!()
    }

    pub async fn get_valid_search(uri: &str) -> Option<ValidSearch> {
        todo!()
    }
}

pub struct SearchRequest {
    site: String,
    query: Option<String>,
    page: u32,
}

pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub cover: String,
}
