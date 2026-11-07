use std::fmt::Display; 
use std::path::PathBuf;

use crate::constants::DATA_DIR;

pub struct History {
    history: Vec<String>,
    current_idx: usize,
    current_search_phrase: String,
}
impl History {
    pub fn load() -> Self {
        let history_path = PathBuf::from(DATA_DIR.clone()).join("history");
        let history_raw = std::fs::read_to_string(history_path).unwrap_or_else(|e| {log::error!("Couldn't read history: {e}"); String::new()});
        let history = history_raw.lines().map(|line| line.to_owned()).collect::<Vec<String>>(); 
        log::info!("{history_raw} | {history:?}");

        History {
            current_idx: history.len(),
            history,
            current_search_phrase: String::new(),
        }
    }

    fn get_next_match(&mut self, phrase: String, direction: Direction) -> Option<String> {
        if self.current_search_phrase != phrase {
            self.current_idx = self.history.len();
            self.current_search_phrase = phrase;
        }

        let mut idx = self.current_idx;

        if direction == Direction::Up {idx = idx.saturating_sub(1)} else {idx = idx.saturating_add(1)}
        while let Some(entry) = self.history.get(idx) {
            if entry.starts_with(&self.current_search_phrase) {
                self.current_idx = idx;
                return Some(entry.clone())
            }
            idx = match direction {
                Direction::Up => idx.saturating_sub(1),
                Direction::Down => idx.saturating_add(1)
            }
        }

        self.current_idx = self.history.len();
        
        None
    }

    pub fn get_above(&mut self, phrase: String) -> Option<String> {
        self.get_next_match(phrase, Direction::Up)
    }

    pub fn get_below(&mut self, phrase: String) -> Option<String> {
        self.get_next_match(phrase, Direction::Down)
    }

    pub fn add_query(&mut self, query: String) {
        self.history.push(query);
        let history = PathBuf::from(DATA_DIR.clone()).join("history");
        if let Err(e) = std::fs::write(history, self.to_string()) {
            log::error!("Failed to write history: {e}");
        } else {
            log::info!("Saved history successfully");
        }
    }
}

impl Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.history.join("\n"))?;
        Ok(())
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down
}