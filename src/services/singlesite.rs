use reqwest::Client;

#[derive(Default)]
pub struct SingleSiteService {
    client: Client
}

impl SingleSiteService {
    pub async fn get_pages(info: &str) -> Vec<String> {
        todo!()
    }
}