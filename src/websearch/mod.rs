use iced::{Task, widget};

use crate::{
    module::{Module, ModuleMessage},
    util, widglets,
};

mod bits;
use bits::SearchResult;
pub use bits::WebMsg;
mod wikipedia;

pub struct Web {
    input_for_results: String,
    cached_results: Vec<SearchResult>, // TODO. Convert to hashmap with input for actual caching
}

impl Default for Web {
    fn default() -> Self {
        Self::new()
    }
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
        self.cached_results.clear();
        self.input_for_results = input.to_string();

        let input_chars = self.input_for_results.chars();
        let first = input_chars.clone().next();
        let search_text = input.trim().to_string();

        match (first, search_text) {
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
        }
    }

    #[cfg(target_os = "linux")]
    fn launch_url(url: &str) {
        util::execute_command_detached::<&str, Vec<&str>>("xdg-open", vec![url], None)
            .expect("Can launch url")
    }

    #[cfg(target_os = "macos")]
    fn launch_url(url: &str) {
        util::execute_command_detached::<&str, Vec<&str>>("open", vec![url], None)
            .expect("Can launch url")
    }
}

impl Module for Web {
    fn view(&self) -> iced::Element<'_, ModuleMessage> {
        log::trace!("Web view function run");
        let elements: Vec<iced::Element<'_, ModuleMessage>> = self
            .cached_results
            .clone()
            .into_iter()
            .map(|result| {
                widglets::listrow(
                    result.title,
                    Some(result.description),
                    Some(ModuleMessage::WebMessage(WebMsg::ResultSelected(
                        result.url,
                    ))), // eww
                    None,
                )
                .into()
            })
            .collect();

        widget::scrollable(widget::column(elements))
            .width(iced::Fill)
            .into()
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
                        Task::none()
                    }
                    WebMsg::ResultSelected(url) => {
                        log::info!("Launching webresult with URL: {url}");
                        Self::launch_url(&url);
                        iced::exit()
                    }
                }
            }
            x => {
                log::trace!("App module received irrelevant msg: {x:?}");
                Task::none()
            }
        }
    }

    fn run(&self) {
        let first = self
            .cached_results
            .first()
            .expect("There are some web results");
        log::info!("first WebResult is: {:?}", first);
        Self::launch_url(&first.url);
    }
}
