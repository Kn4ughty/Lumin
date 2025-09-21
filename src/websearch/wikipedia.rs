use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

use super::bits::{SearchError, SearchResult};

#[derive(Serialize, Deserialize)]
struct WikiResults {
    pages: Vec<WikiResultSingle>,
}

#[derive(Serialize, Deserialize)]
struct WikiResultSingle {
    key: String,
    title: String,
    description: String,
}

impl From<WikiResultSingle> for SearchResult {
    fn from(value: WikiResultSingle) -> Self {
        SearchResult {
            url: format!("https://en.wikipedia.org/wiki/{}", value.key),
            title: value.title,
            description: value.description,
        }
    }
}

pub async fn search(search_text: &str) -> Result<Vec<SearchResult>, SearchError> {
    let url =
        format!("https://en.wikipedia.org/w/rest.php/v1/search/title?q={search_text}&limit=5");

    let client = reqwest::ClientBuilder::new()
        .user_agent("LuminAppLauncher/0.0 (User:Knaughty1234)")
        .build()
        .unwrap();

    let response = client.get(url).send().await.map_err(|e| {
        SearchError::BadResponse(format!("failed to get response: {}", e.to_string()))
    })?;

    let text = response
        .text()
        .await
        .map_err(|e| SearchError::BadResponse(format!("failed to get text: {}", e.to_string())))?;
    log::trace!("text found from wikipedia: {}", text);
    let data: WikiResults = serde_json::from_str(&text).map_err(|e| {
        SearchError::BadResponse(format!("failed to parse from json: {}", e.to_string()))
    })?;

    let parsed = data.pages.into_iter().map(|wr| wr.into()).collect();
    log::debug!("parsed text from wikipedia: {:#?}", parsed);

    Ok(parsed)
}
