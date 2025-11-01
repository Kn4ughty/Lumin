#[derive(Debug, Clone)]
pub enum WebMsg {
    GotResult(Result<Vec<SearchResult>, SearchError>),
    FetchedImage((String, Result<iced::widget::image::Handle, ()>)),
    ResultSelected(String), // URL
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub destination_url: String,
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum SearchError {
    BadResponse(String),
}

impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadResponse(inner) => f.write_str(&format!("Bad Response: {inner}")),
        }
    }
}
