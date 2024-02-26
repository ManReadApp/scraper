use std::collections::HashMap;
use api_structure::error::ApiErr;
use reqwest::Client;
use crate::error::ScrapeError;

#[derive(Default)]
pub struct MetaDataService {
    client: Client,
}

impl MetaDataService {
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
