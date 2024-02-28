use crate::downloader::download;
use crate::pages::hidden;
use crate::services::icon::get_uri;
use crate::services::{config_to_request_builder, Service};
use crate::{ExternalSite, ScrapeError};
use api_structure::error::{ApiErr, ApiErrorType};
use futures::StreamExt;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use api_structure::scrape::ScrapeAccount;

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
    pub async fn get_chapters(
        &self,
        url: &str,
        data: Arc<Vec<ExternalSite>>,
    ) -> Result<(Vec<Info>, Vec<Info>), ScrapeError> {
        let uri = get_uri(&data, url)?;
        let url = modify_url(&self.client, &uri, url).await;
        if let Some(v) = self.services.get(&uri) {
            let req = config_to_request_builder(&self.client, &v.config, &url);
            let html = download(req).await?;
            let fields = v.process(html.as_str());
            post_process(uri.as_str(), fields).map(|v| {
                v.into_iter()
                    .map(|mut v| {
                        if v.url.starts_with("/") {
                            let url_base = url.replace("http://", "").replace("https://", "");
                            v.url = format!(
                                "https://{}{}",
                                url_base
                                    .split_once("/")
                                    .map(|v| v.0.to_string())
                                    .unwrap_or(url_base),
                                v.url
                            );
                        }
                        v
                    })
                    .collect::<Vec<_>>()
            }).map(|v|(v, vec![]))
        } else {
            manual(&self.client, uri.as_str(), &url).await
        }
    }

    pub async fn get_pages(&self, info: Info, acc: Option<ScrapeAccount>) -> Result<Vec<String>, ScrapeError> {
        if let Some(v) = self.services.get(&info.site) {
            let req = config_to_request_builder(&self.client, &v.config, &info.url);
            let html = download(req).await?;
            let fields = v.process(html.as_str());
            post_process_pages(&info.site.as_str(), fields)
        } else {
            manual_pages(&self.client, info, acc).await
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

impl Info {
    pub fn add_title(mut self, title: &Option<String>) -> Self {
        if let Some(v) = title {
            self.titles.push(v.to_string())
        }
        self
    }
}

pub fn parse_episode(s: &str) -> Result<f64, ScrapeError> {
    let re = Regex::new(r"chapter\s+(\d+(\.\d+)?)").unwrap();
    let re2 = Regex::new(r"ch\.\s+(\d+(\.\d+)?)").unwrap();
    if let Some(captured) = re.captures(&s.to_lowercase()) {
        let number_str = &captured[1];
        Ok(number_str.parse()?)
    } else if let Some(captured) = re2.captures(&s.to_lowercase()) {
        let number_str = &captured[1];
        Ok(number_str.parse()?)
    }else {
        Err(ApiErr {
            message: Some("couldnt find chapter number".to_string()),
            cause: None,
            err_type: ApiErrorType::InternalError,
        }
        .into())
    }
}

fn post_process(uri: &str, fields: HashMap<String, String>) -> Result<Vec<Info>, ScrapeError> {
    let err = |len1, len2| {
        if len1 != len2 || len2 == 0 {
            Err(ApiErr {
                message: Some("Ivalid labels/urls".to_string()),
                cause: None,
                err_type: ApiErrorType::InternalError,
            })
        } else {
            Ok(())
        }
    };
    if let Some(urls) = fields.get("urls") {
        let urls: Vec<String> = serde_json::from_str(urls)?;
        let mut res = vec![];
        if let Some(labels) = fields.get("labels") {
            let labels: Vec<String> = serde_json::from_str(labels)?;
            err(labels.len(), urls.len())?;
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
        } else if let Some(episodes) = fields.get("episodes") {
            let episodes: Vec<String> = serde_json::from_str(episodes)?;
            err(episodes.len(), urls.len())?;
            for (i, mut url) in urls.into_iter().enumerate() {
                let title = episodes.get(i).unwrap().replace("-", ".").to_string();
                let episode = title.parse()?;
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

async fn manual(client: &Client, uri: &str, url: &str) -> Result<(Vec<Info>, Vec<Info>), ScrapeError> {
    hidden::multi::manual_info(client, uri, url).await
}

async fn manual_pages(client: &Client, info: Info, acc: Option<ScrapeAccount>) -> Result<Vec<String>, ScrapeError> {
    hidden::multi::manual_pages(client, info, acc).await
}

fn post_process_pages(
    uri: &str,
    fields: HashMap<String, String>,
) -> Result<Vec<String>, ScrapeError> {
    if let Some(v) = fields.get("imgs") {
        let urls: Vec<String> = serde_json::from_str(v)?;
        Ok(urls
            .into_iter()
            .map(|url| url.replace(['\t', '\n'], ""))
            .collect())
    } else {
        hidden::multi::post_process_pages(uri, fields)
    }
}

pub async fn modify_url(client: &Client, uri: &str, url: &str) -> String {
    if let Some(v) = hidden::multi::modify_url(client, uri, url).await {
        v
    } else {
        url.to_string()
    }
}
