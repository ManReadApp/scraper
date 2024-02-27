use crate::downloader::download;
use crate::error::ScrapeError;
use crate::pages::asuratoon::get_first_url;
use crate::services::icon::{get_uri, ExternalSite};
use crate::services::{config_to_request_builder, Service};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct MetaDataService {
    client: Client,
    services: HashMap<String, Service>,
}

impl MetaDataService {
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

    pub async fn get_metadata(
        &self,
        url: &str,
        data: Arc<Vec<ExternalSite>>,
    ) -> Result<MangaInfo, ScrapeError> {
        let uri = get_uri(&data, url)?;
        let url = self.process_url(&uri, url.to_string()).await;
        if let Some(v) = self.services.get(&uri) {
            let req = config_to_request_builder(&self.client, &v.config, url.as_str());
            let html = download(req).await?;
            let fields = v.process(html.as_str());
            post_process(fields)
        } else {
            manual(url).await
        }
    }
    async fn process_url(&self, uri: &str, url: String) -> String {
        if uri == "asuratoon" {
            let html = download(self.client.get(url)).await.unwrap();
            get_first_url(&html).unwrap().to_string()
        } else {
            url
        }
    }
}

pub struct MangaInfo {
    titles: Vec<String>,
    summery: Option<String>,
    genres: Vec<String>,
    other: HashMap<String, String>,
}

fn post_process(values: HashMap<String, String>) -> Result<MangaInfo, ScrapeError> {
    println!("{:?}", values);
    todo!()
}

async fn manual(url: String) -> Result<MangaInfo, ScrapeError> {
    todo!()
}
