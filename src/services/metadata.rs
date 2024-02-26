use std::collections::HashMap;
use reqwest::Client;
use crate::error::ScrapeError;
use crate::services::Service;

#[derive(Default)]
pub struct MetaDataService {
    client: Client,
    services: HashMap<String, Service>,
}

impl MetaDataService {
    pub fn new(service: Vec<Service>) -> Self {
        let services = service.into_iter().map(|v| (v.uri.clone(), v)).collect::<HashMap<_, _>>();
        Self {
            client: Default::default(),
            services,
        }
    }
    fn get_metadata(url: &str) -> Option<Result<MangaInfo, ScrapeError>> {
        todo!()
    }
}

pub struct MangaInfo {
    titles: Vec<String>,
    summery: Option<String>,
    genres: Vec<String>,
    other: HashMap<String, String>,
}
