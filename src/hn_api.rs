use crate::{category::Category, story::Story};
use reqwest::{Client, Error as ReqwestError};
use serde_json::Error as JsonError;
use std::fmt;

const HN_API_BASE: &str = "https://hacker-news.firebaseio.com/v0";
const STORY_LIMIT: usize = 30;

#[derive(Debug)]
pub enum ApiError {
    Request(String),
    Parse(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Request(msg) => write!(f, "Request error: {}", msg),
            ApiError::Parse(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl From<ReqwestError> for ApiError {
    fn from(err: ReqwestError) -> Self {
        ApiError::Request(err.to_string())
    }
}

impl From<JsonError> for ApiError {
    fn from(err: JsonError) -> Self {
        ApiError::Parse(err.to_string())
    }
}

pub async fn fetch_category(client: &Client, category: Category) -> Result<Vec<Story>, ApiError> {
    let endpoint = format!("{}/{}.json", HN_API_BASE, category.api_endpoint());

    let ids: Vec<u64> = client
        .get(&endpoint)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await?
        .json()
        .await?;

    let ids_to_fetch = ids.into_iter().take(STORY_LIMIT).collect::<Vec<_>>();

    let mut stories = Vec::with_capacity(ids_to_fetch.len());

    for id in ids_to_fetch {
        let item_endpoint = format!("{}/item/{}.json", HN_API_BASE, id);

        match client
            .get(&item_endpoint)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => {
                if let Ok(story) = response.json().await {
                    stories.push(story);
                }
            }
            Err(_) => continue,
        }
    }

    Ok(stories)
}

pub fn create_client() -> Client {
    Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap()
}
