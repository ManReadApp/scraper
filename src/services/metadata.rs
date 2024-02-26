use crate::downloader::download;
use crate::error::ScrapeError;
use crate::pages::asuratoon::get_first_url;
use crate::services::icon::{get_uri, ExternalSite};
use crate::services::Service;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{Client, Method};
use std::collections::HashMap;
use std::str::FromStr;
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
    ) -> Option<Result<MangaInfo, ScrapeError>> {
        let uri = get_uri(&data, url)?;
        let url = self.process_url(&uri, url.to_string()).await;
        if let Some(v) = self.services.get(&uri) {
            let method = v.config.get("METHOD").cloned().unwrap_or("GET".to_string());
            let headers = v
                .config
                .iter()
                .map(|(key, value)| {
                    (
                        HeaderName::from_str(key).unwrap(),
                        HeaderValue::from_str(value).unwrap(),
                    )
                })
                .collect();
            let req = self
                .client
                .request(Method::from_str(method.as_str()).unwrap(), url)
                .headers(headers);
            let html = match download(req).await {
                Ok(v) => v,
                Err(e) => return Some(Err(ScrapeError::from(e))),
            };

            let hm = v
                .fields
                .iter()
                .map(|v| (v.name.clone(), v.get(&html)))
                .collect::<HashMap<_, _>>();
            Some(post_process(hm))
        } else {
            Some(manual(url).await)
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
