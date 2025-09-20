use iced::{Task, widget};

use crate::module::{Module, ModuleMessage};

mod bits;
use bits::{SearchError, SearchResult};
mod wikipedia;

#[derive(Debug, Clone)]
pub enum WebMsg {
    GotResult(Result<Vec<SearchResult>, SearchError>),
}

pub struct Web {
    input_for_results: String,
    cached_results: Vec<String>,
}

impl Web {
    pub fn new() -> Self {
        Self {
            input_for_results: String::new(),
            cached_results: vec![],
        }
    }
}

impl Module for Web {
    fn view(&self) -> iced::Element<'_, ModuleMessage> {
        let root = widget::container(widget::text(self.cached_results.concat()));
        root.into()
    }

    fn update(&mut self, msg: ModuleMessage) -> Task<ModuleMessage> {
        match msg {
            ModuleMessage::TextChanged(input) => {
                if self.input_for_results != input {
                    self.cached_results.clear();
                    self.input_for_results = input.to_string();

                    let input_chars = self.input_for_results.chars();
                    let first = input_chars.clone().next();
                    let search_text = input.trim().to_string();
 
                    return match (first, search_text) {
                        // get first char
                        (Some('w'), search_text) => {
                            log::info!("wikipedia time!");
                            Task::perform(
                                async move { wikipedia::search(&search_text).await },
                                |r| ModuleMessage::WebMessage(WebMsg::GotResult(r)),
                            )
                        }
                        (None, _) => {
                            log::info!("Did not match in web searcher");
                            Task::none()
                        }
                        _ => {
                            log::info!("unknown search prefix");
                            Task::none()
                        }
                    };
                }
                Task::none()
            }
            ModuleMessage::WebMessage(inner) => {
                log::info!("received a webMessage yay!!! inner {inner:?}");
                match inner {
                    WebMsg::GotResult(r) => log::info!("message was result: {r:?}")
                }
                Task::none()
            }
            // _ => Task::none(),
        }
    }

    fn run(&self) {
        println!("first result is: {:?}", self.cached_results.first())
    }
}
