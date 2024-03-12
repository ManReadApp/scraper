use crate::extractor::SearchServiceScrapeData;
use crate::pages::{anilist, kitsu};
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
        search: ExternalSearchData,
    ) -> Result<Vec<ScrapeSearchResult>, ScrapeError> {
        match uri {
            "anilist" => anilist::search(&self.client, &search.get_simple()?).await,
            "kitsu" => kitsu::search(&self.client, search.get_simple()?).await,
            _ => Err(ScrapeError::input_error("uri does not exist")),
        }
    }
}
