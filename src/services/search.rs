use crate::services::Service;
use reqwest::Client;
use std::collections::HashMap;

#[derive(Default)]
pub struct SearchService {
    client: Client,
    services: HashMap<String, Service>,
}

impl SearchService {
    pub fn new(service: Vec<Service>) -> Self {
        let services = service
            .into_iter()
            .map(|v| (v.uri.clone(), v))
            .collect::<HashMap<_, _>>();
        Self {
            client: Default::default(),
            services,
        }
    }
    pub fn sites() -> Vec<String> {
        todo!()
    }

    pub async fn search(search: SearchService) -> Vec<SearchResult> {
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
