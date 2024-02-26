use std::collections::HashMap;
use std::fs::{File, read_dir};
use std::io;
use std::io::{BufRead, read_to_string, Write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::error::ScrapeError;
use crate::extractor::parser::Field;
use crate::services::metadata::MetaDataService;
use crate::services::multisite::MultiSiteService;
use crate::services::search::SearchService;
use crate::services::singlesite::SingleSiteService;

pub mod search;
pub mod metadata;
pub mod multisite;
pub mod singlesite;
pub mod icon;

#[test]
fn test() {
    init(PathBuf::from("test")).unwrap();
}

struct Service {
    fields: Vec<Field>,
    uri: String,
    config: HashMap<String, String>,
}

pub fn init(root_folder: PathBuf) -> Result<(MultiSiteService, SingleSiteService, SearchService, MetaDataService), ScrapeError> {
    let folder = root_folder.join("external");
    let mut search = vec![];
    let mut meta = vec![];
    let mut multi = vec![];
    let mut single = vec![];
    for entry in read_dir(&folder)? {
        let path = entry?.path();
        if path.is_file() {
            let name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
            if !name.starts_with(".") && name.ends_with(".scraper") {
                let file = File::open(path.as_path())?;
                let reader = io::BufReader::new(file);
                let mut lines = reader.lines();
                if let Some(Ok(first_line)) = lines.next() {
                    let header: Header = serde_json::from_str(&format!("{}{}{}", '{', first_line, '}'))?;
                    let text = lines.collect::<Vec<String>>().join("\n");
                    let v = Field::parse(text.as_str());
                    let config = if let Some(file) = header.request_config {
                        let text = read_to_string(folder.join(file))?;
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
    Ok((MultiSiteService::new(multi), SingleSiteService::new(single),SearchService::new(search),MetaDataService::new(meta)))
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