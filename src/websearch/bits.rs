use crate::module::ModuleMessage;
use crate::widglets;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub url: String,
    pub title: String,
    pub description: String,
}

impl From<SearchResult> for iced::Element<'_, ModuleMessage> {
    fn from(value: SearchResult) -> Self {
        widglets::listrow(value.title, Some(value.description), Some("".into())).into()
    }
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
