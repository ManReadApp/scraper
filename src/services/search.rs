use reqwest::{Client};

#[derive(Default)]
pub struct SearchService {
    client: Client,
}

impl SearchService {
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