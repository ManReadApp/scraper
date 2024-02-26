use crate::services::Service;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default)]
pub struct MultiSiteService {
    client: Client,
    services: HashMap<String, Service>,
}

impl MultiSiteService {
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
    pub async fn get_chapters(url: String) -> Vec<Info> {
        todo!()
    }

    pub async fn get_pages(info: Info) -> Vec<String> {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub site: String,
    pub url: String,
    pub titles: Vec<String>,
    pub episode: f64,
    pub account: Option<i64>,
}
