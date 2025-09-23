use iced::Task;

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

impl Web {
    pub fn new() -> Self {
        Self {
            input_for_results: String::new(),
            cached_results: vec![],
        }
    }

    /// Split up just bc the indentation was getting to be too much
    fn handle_text_change(&mut self, input: String) -> Task<ModuleMessage> {
        if self.input_for_results == input {
            // Text hasnt changed. Do nothing. Shouldn't happen, but no need to spam API's
            // redundantly
            log::warn!(
                "The web search text_change func was run, even though the input text hasnt changed since the last time. This is odd. Contact Dev with replication instructions"
            );
            return Task::none();
        }

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

    #[cfg(target_os = "linux")]
    fn launch_url(url: &str) {
        // TODO. Make your own launcher thing because it will be faster.
        util::execute_command_detached::<&str, Vec<&str>>("xdg-open", vec![url], None)
            .expect("Can launch url")
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
                        Task::none()
                    }
                    WebMsg::ResultSelected(url) => {
                        log::info!("Launching webresult with URL: {url}");
                        Self::launch_url(&url);
                        iced::exit()
                    }
                }
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
