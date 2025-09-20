
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub url: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum SearchError {
    BadResponse(String)
}

impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("blah blah blah")
    }
}
