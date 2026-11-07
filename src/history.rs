use std::fmt::Display; 
use std::path::PathBuf;

use crate::constants::DATA_DIR;

pub struct History {
    history: Vec<String>,
    current_search_phrase: String,
    direction: Option<Direction>,
    current_search_filter: Vec<String>
}
impl History {
    pub fn load() -> Self {
        let history_path = PathBuf::from(DATA_DIR.clone()).join("history");
        let history_raw = std::fs::read_to_string(history_path).unwrap_or(String::new());
        let history = history_raw.lines().map(|line| line.to_owned()).collect::<Vec<String>>(); 
        log::info!("{history_raw} | {history:?}");

        History {
            history,
            current_search_phrase: String::new(),
            direction: None,
            current_search_filter: vec![],
        }
    }

    fn get_next_match(&mut self, phrase: String, direction: Direction) -> Option<String> {
        if self.current_search_phrase != phrase || self.direction != Some(direction) {
            // If it's the first time we index history, we need to grab the last item.
            // Otherwise, (because switching direction) we should index an extra time
            // since we're moving
            // idk explain this better when im saner
            let mut is_first_tap = false;
            if self.direction.is_none() || self.direction == Some(direction) {
                is_first_tap = true;
            }
            self.current_search_phrase = phrase;
            self.direction = Some(direction);
            let mut search = self.history.clone();

            if self.direction == Some(Direction::Down) {
                search.reverse();
            }

            let search = search[..dbg!(self.history.len()-self.current_search_filter.len())]
                .iter()
                .filter(|item| item.starts_with(&self.current_search_phrase))
                .cloned()
                .collect::<Vec<String>>();

            self.current_search_filter = search;

            if !is_first_tap {let _ = self.current_search_filter.pop();}
            log::info!("{:?}", self.current_search_filter)
        }
        self.current_search_filter.pop()
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
        let _ = std::fs::write(history, self.to_string());
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