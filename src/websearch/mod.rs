use iced::{Task, widget};

use crate::{
    module::{Module, ModuleMessage},
    widglets,
};

mod bits;
use bits::{SearchError, SearchResult};
mod wikipedia;

#[derive(Debug, Clone)]
pub enum WebMsg {
    GotResult(Result<Vec<SearchResult>, SearchError>),
}

pub struct Web {
    input_for_results: String,
    cached_results: Vec<SearchResult>,
}

impl Web {
    pub fn new() -> Self {
        Self {
            input_for_results: String::new(),
            cached_results: vec![],
        }
    }

    /// Split up just bc the indentation was getting to be too much
    fn handle_text_change(&mut self, input: String) -> Task<ModuleMessage> {
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
                    // trim first character. TODO. Dont hardcode
                    Task::perform(
                        async move { wikipedia::search(&search_text[1..]).await },
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
}

impl Module for Web {
    fn view(&self) -> iced::Element<'_, ModuleMessage> {
        log::trace!("Web view function run");
        let root = widglets::listbox(self.cached_results.clone());
        root.into()
    }

    fn update(&mut self, msg: ModuleMessage) -> Task<ModuleMessage> {
        match msg {
            ModuleMessage::TextChanged(input) => self.handle_text_change(input),
            ModuleMessage::WebMessage(inner) => {
                log::trace!("received a webMessage yay!!! inner {inner:?}");
                match inner {
                    WebMsg::GotResult(r) => {
                        log::trace!("message was result: {r:?}");
                        match r {
                            Ok(o) => self.cached_results = o,
                            Err(e) => {
                                log::warn!("WebResult was error! {e:?}")
                            }
                        }
                    }
                }
                Task::none()
            }
        }
    }

    fn run(&self) {
        println!("first result is: {:?}", self.cached_results.first())
    }
}
