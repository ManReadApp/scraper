use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct MultiSiteService {
    client: Client
}

impl MultiSiteService{
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