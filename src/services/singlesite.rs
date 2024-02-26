use crate::services::Service;
use reqwest::Client;
use std::collections::HashMap;

#[derive(Default)]
pub struct SingleSiteService {
    client: Client,
    services: HashMap<String, Service>,
}

impl SingleSiteService {
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
    pub async fn get_pages(info: &str) -> Vec<String> {
        todo!()
    }
}
