use serde::Deserialize;

use super::bits::{SearchError, SearchResult};

#[derive(Deserialize)]
struct WikiResults {
    pages: Vec<WikiResultSingle>,
}

#[derive(Deserialize, Debug)]
struct WikiResultSingle {
    key: String,
    title: String,
    #[serde(default)]
    description: String,
    #[serde(rename = "thumbnail")]
    raw_thumb: Option<RawThumb>,
}

#[derive(Deserialize, Debug)]
struct RawThumb {
    url: String,
}

impl From<WikiResultSingle> for SearchResult {
    fn from(value: WikiResultSingle) -> Self {
        log::trace!("raw WikiResultSingle is: {value:?}");
        SearchResult {
            destination_url: format!("https://en.wikipedia.org/wiki/{}", value.key),
            title: value.title,
            description: value.description,
            image_url: value.raw_thumb.map(|rt| format!("https:{}", rt.url)),
        }
    }
}

// the search_text is passed back through, so that the later parts of the task_perform can keep
// ownership
pub async fn search(
    client: &reqwest::Client,
    search_text: &str,
) -> Result<Vec<SearchResult>, SearchError> {
    let url =
        format!("https://en.wikipedia.org/w/rest.php/v1/search/title?q={search_text}&limit=5");

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| SearchError::BadResponse(format!("failed to get response: {}", e)))?;

    let text = response
        .text()
        .await
        .map_err(|e| SearchError::BadResponse(format!("failed to get text: {}", e)))?;

    log::trace!("text found from wikipedia: {}", text);

    let data: WikiResults = serde_json::from_str(&text)
        .map_err(|e| SearchError::BadResponse(format!("failed to parse from json: {}", e)))?;

    let parsed = data.pages.into_iter().map(|result| result.into()).collect();
    log::debug!("parsed text from wikipedia: {:#?}", parsed);

    Ok(parsed)
}
