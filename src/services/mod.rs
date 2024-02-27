use crate::error::ScrapeError;
use crate::extractor::parser::Field;
use crate::services::metadata::MetaDataService;
use crate::services::multisite::MultiSiteService;
use crate::services::search::SearchService;
use crate::services::singlesite::SingleSiteService;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io;
use std::io::{read_to_string, BufRead, Write};
use std::path::PathBuf;
use std::str::FromStr;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;

pub mod icon;
pub mod metadata;
pub mod multisite;
pub mod search;
pub mod singlesite;



struct Service {
    fields: Vec<Field>,
    uri: String,
    config: HashMap<String, String>,
}

impl Service {
    fn process(&self, html: &str) -> HashMap<String, String> {
        self.fields
            .iter()
            .filter_map(|v| v.get(html).map(|res|(v.name.clone(), res)))
            .collect::<HashMap<_, _>>()
    }
}

pub fn init(
    root_folder: PathBuf,
) -> Result<
    (
        MultiSiteService,
        SingleSiteService,
        SearchService,
        MetaDataService,
    ),
    ScrapeError,
> {
    let folder = root_folder.join("external");
    let mut search = vec![];
    let mut meta = vec![];
    let mut multi = vec![];
    let mut single = vec![];
    for entry in read_dir(&folder)? {
        let path = entry?.path();
        if path.is_file() {
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            if !name.starts_with(".") && name.ends_with(".scraper") {
                let file = File::open(path.as_path())?;
                let reader = io::BufReader::new(file);
                let mut lines = reader.lines();
                if let Some(Ok(first_line)) = lines.next() {
                    let header: Header =
                        serde_json::from_str(&format!("{}{}{}", '{', first_line, '}'))?;
                    let text = lines
                        .collect::<Result<Vec<String>, _>>()
                        .unwrap()
                        .join("\n");
                    let v = Field::parse(text.as_str());
                    let config = if let Some(file) = header.request_config {
                        let text = read_to_string(File::open(folder.join(file)).unwrap())?;
                        serde_json::from_str(&text)?
                    } else {
                        HashMap::new()
                    };
                    let service = Service {
                        fields: v,
                        uri: header.uri,
                        config,
                    };
                    match header.kind {
                        Kind::SingleSiteScraper => single.push(service),
                        Kind::MultiSiteScraper => multi.push(service),
                        Kind::Search => search.push(service),
                        Kind::Metadata => meta.push(service),
                    }
                }
            }
        }
    }
    Ok((
        MultiSiteService::new(multi),
        SingleSiteService::new(single),
        SearchService::new(search),
        MetaDataService::new(meta),
    ))
}

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    uri: String,
    kind: Kind,
    request_config: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Kind {
    SingleSiteScraper,
    MultiSiteScraper,
    Search,
    Metadata,
}

pub fn hashmap_to_struct<T: DeserializeOwned>(hm: HashMap<String, String>) -> serde_json::Result<T> {
    serde_json::from_str(&serde_json::to_string(&hm).unwrap())
}

pub fn config_to_request_builder(client: &Client, config: &HashMap<String, String>, url: &str)-> RequestBuilder {
    let method = config.get("METHOD").cloned().unwrap_or("GET".to_string());
    let headers = config
        .iter()
        .map(|(key, value)| {
            (
                HeaderName::from_str(key).unwrap(),
                HeaderValue::from_str(value).unwrap(),
            )
        })
        .collect();
    client
        .request(Method::from_str(method.as_str()).unwrap(), url)
        .headers(headers)
}
