use crate::extractor::SearchServiceScrapeData;
use crate::ScrapeError;
use api_structure::scraper::{ExternalSearchData, ScrapeSearchResult};
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

    pub fn sites(&self) -> Vec<String> {
        let mut keys = vec![
            "kitsu".to_string(),
            "anilist".to_string(),
            "anime-planet".to_string(),
        ];
        keys.append(&mut self.services.keys().cloned().collect::<Vec<_>>());
        keys
    }

    pub async fn search(
        &self,
        uri: &str,
        _search: ExternalSearchData,
    ) -> Result<Vec<ScrapeSearchResult>, ScrapeError> {
        if !self.sites().contains(&uri.to_owned()) {
            return Err(ScrapeError::input_error("uri does not exist"));
        }
        todo!()
    }
}
