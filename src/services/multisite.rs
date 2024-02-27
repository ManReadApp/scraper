use crate::services::{config_to_request_builder, Service};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use api_structure::error::{ApiErr, ApiErrorType};
use futures::StreamExt;
use regex::Regex;
use serde_json::Value;
use crate::downloader::download;
use crate::{ExternalSite, ScrapeError};
use crate::pages::hidden;
use crate::services::icon::get_uri;

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
    pub async fn get_chapters(&self, url: &str, data: Arc<Vec<ExternalSite>>) -> Result<Vec<Info>, ScrapeError> {
        let uri = get_uri(&data, url)?;
        if let Some(v) = self.services.get(&uri) {
            let req = config_to_request_builder(&self.client, &v.config, url);
            let html = download(req).await?;
            let fields = v.process(html.as_str());
            post_process(uri.as_str(), fields).map(|v| v.into_iter().map(|mut v| {
                if v.url.starts_with("/") {
                    let url_base = url.replace("http://", "").replace("https://", "");
                    v.url = format!("https://{}{}", url_base.split_once("/").map(|v| v.0.to_string()).unwrap_or(url_base), v.url);
                }
                v
            }).collect())
        } else {
            manual(uri.as_str(), url).await
        }
    }

    pub async fn get_pages(&self, info: Info) -> Result<Vec<String>, ScrapeError> {
        if let Some(v) = self.services.get(&info.site) {
            let req = config_to_request_builder(&self.client, &v.config, &info.url);
            let html = download(req).await?;
            let fields = v.process(html.as_str());
            let value = serde_json::from_str::<Value>(fields.get("img_json").unwrap()).unwrap()["imageFiles"][1].clone();
            let value = value.as_str().unwrap_or_default();
            let v:Vec<Vec<Value>> = serde_json::from_str(value).unwrap();
            let v:Vec<_>= v.into_iter().map(|v|v[1].as_str().unwrap_or_default().to_string()).collect();
            post_process_pages(&info.site.as_str(), fields)
        } else {
            manual_pages(info)
        }
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

fn parse_episode(s: &str) -> Result<f64, ScrapeError> {
    let re = Regex::new(r"chapter\s+(\d+(\.\d+)?)").unwrap();
    if let Some(captured) = re.captures(&s.to_lowercase()) {
        let number_str = &captured[1];
        Ok(number_str.parse()?)
    } else {
        Err(ApiErr {
            message: Some("couldnt find chapter number".to_string()),
            cause: None,
            err_type: ApiErrorType::InternalError,
        }.into())
    }
}

fn post_process(uri: &str, fields: HashMap<String, String>) -> Result<Vec<Info>, ScrapeError> {
    if let Some(urls) = fields.get("urls") {
        let urls: Vec<String> = serde_json::from_str(urls)?;
        if let Some(labels) = fields.get("labels") {
            let mut res = vec![];
            let labels: Vec<String> = serde_json::from_str(labels)?;
            if labels.len() != urls.len() || urls.is_empty() {
                return Err(ApiErr {
                    message: Some("Ivalid labels/urls".to_string()),
                    cause: None,
                    err_type: ApiErrorType::InternalError,
                }.into());
            }
            for (i, mut url) in urls.into_iter().enumerate() {
                let title = labels.get(i).unwrap().to_string();
                let episode = parse_episode(title.as_str())?;
                res.push(Info {
                    site: uri.to_string(),
                    url,
                    titles: vec![title],
                    episode,
                    account: None,
                })
            }
            return Ok(res);
        }
    }
    hidden::multi::post_process_info(uri, fields)
}

async fn manual(uri: &str, url: &str) -> Result<Vec<Info>, ScrapeError>{
    hidden::multi::manual_info(uri, url).await
}
async fn manual_pages(info: Info) -> Result<Vec<String>, ScrapeError>{
    hidden::multi::manual_pages(info).await
}

fn post_process_pages(uri: &str, fields: HashMap<String, String>) -> Result<Vec<String>, ScrapeError> {
    hidden::multi::post_process_pages(uri, fields)
}
