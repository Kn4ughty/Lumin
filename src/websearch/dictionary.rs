use super::{SearchError, SearchResult};
use serde::Deserialize;

#[derive(Debug, Clone)]
enum DictMessage {
    ResultFetched(String, Response),
}
#[derive(Clone, Debug, Deserialize)]
struct Response {
    // phonetic: String,
    // origin: String,
    meanings: Vec<Meaning>,
}

#[derive(Deserialize, Debug, Clone)]
struct Meaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    definitions: Vec<Defintion>,
}

#[derive(Deserialize, Debug, Clone)]
struct Defintion {
    definition: String,
    // example: String,
}

impl From<&Response> for super::SearchResult {
    fn from(value: &Response) -> Self {
        Self {
            destination_url: String::from("n/a"),
            title: {
                // this makes me go *_*
                let mut o = String::new();
                value
                    .meanings
                    .iter()
                    .map(|m| {
                        let mut s = String::new();
                        m.definitions
                            .iter()
                            .map(|d| d.definition.clone())
                            .for_each(|d| s += &d);
                        s
                    })
                    .for_each(|t| o += &t);
                o
            },
            description: String::from("hugjfl"),
            image_url: None,
        }
    }
}

pub async fn search(
    client: &reqwest::Client,
    search_text: &str,
) -> Result<Vec<SearchResult>, SearchError> {
    let url = format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        search_text
    );

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| SearchError::BadResponse(format!("failed to get response: {}", e)))?;

    let text = response
        .text()
        .await
        .map_err(|e| SearchError::BadResponse(format!("failed to get text: {}", e)))?;

    log::trace!("text found from dictionary: {}", text);

    let data: Vec<Response> = serde_json::from_str(&text)
        .map_err(|e| SearchError::BadResponse(format!("failed to parse from json: {}", e)))?;

    let data = data.iter().map(|f| f.into()).collect();

    // let parsed = data.pages.into_iter().map(|result| result.into()).collect();
    // log::debug!("parsed text from wikipedia: {:#?}", parsed);

    Ok(data)
}
