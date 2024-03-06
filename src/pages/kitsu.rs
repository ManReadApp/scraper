use crate::downloader::download;
use crate::services::metadata::ItemOrArray;
use crate::ScrapeError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub async fn get_data(
    client: &Client,
    url: &str,
) -> Result<HashMap<String, ItemOrArray>, ScrapeError> {
    let mut slug = url
        .split_once("kitsu.io/manga/")
        .map(|v| v.1)
        .ok_or(ScrapeError::input_error("not a valid url"))?;
    if slug.contains("/") {
        slug = slug.split_once("/").unwrap().0;
    }
    let url = format!("https://kitsu.io/api/edge/manga?fields%5Bcategories%5D=slug%2Ctitle&filter%5Bslug%5D={slug}&include=categories,genres");
    let text = download(client.get(url)).await?;
    let mut parsed: MangaResponse = serde_json::from_str(&text)?;
    let data = parsed.data.remove(0);
    let mut hm = HashMap::new();
    let mut titles = vec![format!(
        "canonical_title: {}",
        data.attributes.canonical_title
    )];
    for (typ, title) in data.attributes.titles {
        titles.push(format!("{}: {}", typ, title));
    }

    for title in data.attributes.abbreviated_titles {
        titles.push(format!("unknown: {}", title))
    }
    hm.insert(
        "description",
        ItemOrArray::Item(data.attributes.description),
    );
    hm.insert("type", ItemOrArray::Item(data.r#type));
    hm.insert("subtype", ItemOrArray::Item(data.attributes.subtype));
    hm.insert(
        "cover",
        ItemOrArray::Item(data.attributes.cover_image.original),
    );
    hm.insert(
        "poster",
        ItemOrArray::Item(data.attributes.poster_image.original),
    );
    hm.insert("age_rating", ItemOrArray::Item(data.attributes.age_rating));
    hm.insert("status", ItemOrArray::Item(data.attributes.status));
    hm.insert("start_date", ItemOrArray::Item(data.attributes.start_date));
    hm.insert(
        "serialization",
        ItemOrArray::Item(data.attributes.serialization),
    );
    hm.insert(
        "tags",
        ItemOrArray::Array(
            parsed
                .included
                .into_iter()
                .map(|v| v.attributes.get_name())
                .collect::<HashSet<_>>()
                .into_iter()
                .collect(),
        ),
    );
    hm.insert("titles", ItemOrArray::Array(titles));
    Ok(hm.into_iter().map(|v| (v.0.to_string(), v.1)).collect())
}

#[derive(Serialize, Deserialize)]
struct MangaResponse {
    pub data: Vec<Info>,
    pub included: Vec<Data>,
}

#[derive(Serialize, Deserialize)]
struct Info {
    #[serde(rename = "type")]
    pub r#type: String,
    pub attributes: MangaAttributes,
}

#[derive(Serialize, Deserialize)]
struct MangaAttributes {
    pub description: String,
    pub titles: HashMap<String, String>,
    #[serde(rename = "canonicalTitle")]
    pub canonical_title: String,
    #[serde(rename = "abbreviatedTitles")]
    pub abbreviated_titles: Vec<String>,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "ageRating")]
    pub age_rating: String,
    pub subtype: String,
    pub status: String,
    #[serde(rename = "posterImage")]
    pub poster_image: Image,
    #[serde(rename = "coverImage")]
    pub cover_image: Image,
    pub serialization: String,
}

#[derive(Serialize, Deserialize)]
struct Image {
    pub original: String,
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub attributes: Attributes1,
}

#[derive(Serialize, Deserialize)]
struct Attributes1 {
    name: Option<String>,
    slug: String,
    title: Option<String>,
}

impl Attributes1 {
    fn get_name(self) -> String {
        if let Some(v) = self.name {
            v
        } else if let Some(v) = self.title {
            v
        } else {
            self.slug
        }
    }
}
