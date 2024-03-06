use crate::downloader::download;
use crate::services::metadata::ItemOrArray;
use crate::ScrapeError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

const QUERY: &str = "
query ($id: Int) { # Define which variables will be used in the query (id)
  Media (id: $id, type: MANGA) { # Insert our variables into the query arguments (id) (type: ANIME is hard-coded in the query)
    title {
      romaji
      english
      native
    }
    coverImage {
      extraLarge
    }
    bannerImage
    startDate {
      year
      month
      day
    }
    endDate {
      year
      month
      day
    }
    description
    type
    format
    genres
    status(version:2)
    source(version:3)
    synonyms
    isAdult
    countryOfOrigin
    isLicensed
    tags {
      name
    }
    studios {
      edges {
        isMain node{
          name
        }
      }
    }
    relations {
      edges {
        relationType(version:2)node {
          title{
            userPreferred
          }
          type
          coverImage {
            extraLarge
          }
        }
      }
    }
  }
}
";
pub async fn get_data(
    client: &Client,
    url: &str,
) -> Result<HashMap<String, ItemOrArray>, ScrapeError> {
    let mut id = url
        .split_once("anilist.co/manga/")
        .ok_or(ScrapeError::input_error("Invalid url"))?
        .1;
    if id.contains("/") {
        id = id.split_once("/").unwrap().0;
    }
    let json = json!({"query": QUERY, "variables": {"id": id}});
    let resp = download(
        client
            .post("https://graphql.anilist.co/")
            .header("Accept", "application/json")
            .json(&json),
    )
    .await?;
    let parsed: InfoResponse = serde_json::from_str(&resp)?;
    let mut media = parsed.data.media;
    let mut hm = HashMap::new();
    hm.insert("cover", ItemOrArray::Item(media.cover_image.extra_large));
    if let Some(banner_image) = media.banner_image {
        hm.insert("banner", ItemOrArray::Item(banner_image));
    }
    let mut titles = vec![
        format!("Native: {}", media.title.native),
        format!("Rative: {}", media.title.romaji),
        format!("English: {}", media.title.english),
    ];
    for synonym in media.synonyms {
        titles.push(format!("Unknown: {}", synonym));
    }
    hm.insert("titles", ItemOrArray::Array(titles));
    let mut tags: Vec<String> = media.tags.into_iter().map(|v| v.name).collect();
    tags.append(&mut media.genres);
    hm.insert("tags", ItemOrArray::Array(tags));
    hm.insert("licensed", ItemOrArray::Item(media.is_licensed.to_string()));
    hm.insert("adult", ItemOrArray::Item(media.is_adult.to_string()));
    hm.insert(
        "country_of_origin",
        ItemOrArray::Item(media.country_of_origin),
    );
    hm.insert("description", ItemOrArray::Item(media.description));
    if let Some(start_date) = media.start_date.display() {
        hm.insert("start_date", ItemOrArray::Item(start_date));
    }
    if let Some(end_date) = media.end_date.display() {
        hm.insert("end_date", ItemOrArray::Item(end_date));
    }
    hm.insert("format", ItemOrArray::Item(media.format));
    hm.insert("status", ItemOrArray::Item(media.status));
    hm.insert("source", ItemOrArray::Item(media.source));
    hm.insert("type", ItemOrArray::Item(media.r#type));
    let mut relations = vec![];
    for relation in media.relations.edges {
        relations.push(format!(
            "{} {}: {}",
            relation.relation_type, relation.node.r#type, relation.node.title.user_preferred,
        ));
    }
    hm.insert("relations", ItemOrArray::Array(relations));
    Ok(hm.into_iter().map(|v| (v.0.to_string(), v.1)).collect())
}

#[derive(Serialize, Deserialize)]
struct Title1 {
    #[serde(rename = "userPreferred")]
    pub user_preferred: String,
}

#[derive(Serialize, Deserialize)]
struct Node {
    pub title: Title1,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "coverImage")]
    pub cover_image: CoverImage,
}

#[derive(Serialize, Deserialize)]
struct Relation {
    #[serde(rename = "relationType")]
    pub relation_type: String,
    pub node: Node,
}

#[derive(Serialize, Deserialize)]
struct Relations {
    pub edges: Vec<Relation>,
}

#[derive(Serialize, Deserialize)]
struct Studios {
    pub edges: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
struct Tag {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct Media {
    pub title: Title,
    #[serde(rename = "coverImage")]
    pub cover_image: CoverImage,
    #[serde(rename = "bannerImage")]
    pub banner_image: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: Date,
    #[serde(rename = "endDate")]
    pub end_date: Date,
    pub description: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub format: String,
    pub genres: Vec<String>,
    pub status: String,
    pub source: String,
    pub synonyms: Vec<String>,
    #[serde(rename = "isAdult")]
    pub is_adult: bool,
    #[serde(rename = "countryOfOrigin")]
    pub country_of_origin: String,
    #[serde(rename = "isLicensed")]
    pub is_licensed: bool,
    pub tags: Vec<Tag>,
    pub studios: Studios,
    pub relations: Relations,
}

#[derive(Serialize, Deserialize)]
struct InfoResponse {
    pub data: Data,
}
#[derive(Serialize, Deserialize)]
struct Data {
    #[serde(rename = "Media")]
    pub media: Media,
}

#[derive(Serialize, Deserialize)]
struct Title {
    pub romaji: String,
    pub english: String,
    pub native: String,
}

#[derive(Serialize, Deserialize)]
struct CoverImage {
    #[serde(rename = "extraLarge")]
    pub extra_large: String,
}

#[derive(Serialize, Deserialize)]
struct Date {
    pub year: Option<i64>,
    pub month: Option<i64>,
    pub day: Option<i64>,
}

impl Date {
    fn display(&self) -> Option<String> {
        if let Some(year) = self.year {
            if let Some(month) = self.month {
                if let Some(day) = self.day {
                    return Some(format!("{}-{}-{}", year, month, day));
                }
            }
        }
        None
    }
}
