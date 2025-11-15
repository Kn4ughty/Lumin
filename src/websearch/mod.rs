use iced::{Task, widget};
use std::collections::HashMap;

use crate::{
    module::{Module, ModuleMessage},
    util,
    websearch::bits::SearchError,
    widglets,
};

mod bits;
use bits::SearchResult;
pub use bits::WebMsg;
mod wikipedia;

#[derive(Debug)]
pub struct Web {
    input_for_results: String,
    cached_results: HashMap<String, Vec<SearchResult>>,

    // The memory cost of this isnt actually that bad. Each image is just a couple kB each since
    // they are very small thumbnails. It only increased a few mB over like 10s of usage
    image_hashmap: HashMap<String, widget::image::Handle>,
    client: reqwest::Client,
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
            cached_results: HashMap::new(),
            image_hashmap: HashMap::new(),
            client: reqwest::ClientBuilder::new()
                // https://foundation.wikimedia.org/wiki/Special:MyLanguage/Policy:User-Agent_policy
                .user_agent("LuminAppLauncher/0.0 (https://github.com/Kn4ughty)")
                .build()
                .expect("Can create web client"),
        }
    }

    /// Split up just bc the indentation was getting to be too much
    fn handle_text_change(&mut self, input: String) -> Task<ModuleMessage> {
        self.input_for_results = input.to_string();

        // Is this search text already in the cache
        if self.cached_results.contains_key(&self.input_for_results) {
            return Task::none();
        };

        let input_chars = self.input_for_results.chars();
        let first = input_chars.clone().next();
        let search_text = input.trim().to_string();

        match (first, search_text) {
            // get first char
            (Some('w'), search_text) => {
                log::debug!("wikipedia time!");
                let client = self.client.clone();

                let full_text = self.input_for_results.clone();
                // trim first character. TODO. Dont hardcode
                let trimmed_text = search_text[1..].to_owned();
                Task::perform(
                    async move {
                        let res = wikipedia::search(&client, trimmed_text.as_str()).await;
                        // this little tuple maneuver is cool
                        (full_text, res)
                    },
                    |r| ModuleMessage::WebMessage(WebMsg::GotResult(r.0, r.1)),
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

    fn handle_getting_image(client: reqwest::Client, input: SearchResult) -> Task<ModuleMessage> {
        log::trace!("handle_getting_image ran. SR: {input:?}");

        let Some(url) = input.image_url else {
            return Task::none();
        };

        Task::perform(
            async move { (url.clone(), Self::get_image(client, &url).await) },
            |r| {
                let image = match r.1 {
                    Ok(bytes) => Ok(widget::image::Handle::from_bytes(bytes)),
                    Err(e) => {
                        log::warn!("Could not get image from url: {e:?}");
                        Err(())
                    }
                };
                ModuleMessage::WebMessage(WebMsg::FetchedImage((r.0, image)))
            },
        )
    }

    async fn get_image(
        client: reqwest::Client,
        url: &str,
    ) -> Result<iced::advanced::image::Bytes, SearchError> {
        let response = client.get(url).send().await.map_err(|e| {
            SearchError::BadResponse(format!(
                "failed to get response for image: {}. url: {url}",
                e
            ))
        })?;
        let bytes = response
            .bytes()
            .await
            .map_err(|e| SearchError::BadResponse(format!("failed to get bytes: {}", e)))?;
        Ok(bytes)
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
    fn description(&self) -> String {
        String::from("Quick access to search different websites")
    }

    fn view(&self) -> iced::Element<'_, ModuleMessage> {
        log::debug!("Web view function run");
        log::trace!("Self. {self:#?}");
        let empty = Vec::new();
        let results = self
            .cached_results
            .get(&self.input_for_results)
            .unwrap_or(&empty);

        let elements: Vec<iced::Element<'_, ModuleMessage>> = results
            .clone()
            .into_iter()
            .map(|result| {
                log::trace!("Viewing webresult {:?}", result);

                let image = match result.image_url {
                    None => None,
                    Some(url) => self.image_hashmap.get(&url).cloned(),
                };

                widglets::ListRow::new(result.title)
                    .subtext(result.description)
                    .on_activate(ModuleMessage::WebMessage(WebMsg::ResultActivated(
                        result.destination_url,
                    )))
                    .optional_icon(image)
                    .icon_background(iced::Color::WHITE)
                    .into()
            })
            .collect();

        widget::scrollable(widget::column(elements))
            .style(|theme, status| {
                let mut base_widget = widget::scrollable::default(theme, status);
                base_widget.vertical_rail = widget::scrollable::Rail {
                    background: None,
                    border: iced::Border {
                        color: iced::Color::TRANSPARENT,
                        width: 0.0,
                        ..Default::default()
                    },
                    scroller: widget::scrollable::Scroller {
                        color: iced::Color::TRANSPARENT,
                        border: base_widget.horizontal_rail.border,
                    },
                };
                base_widget
            })
            .width(iced::Fill)
            .into()
    }

    fn update(&mut self, msg: ModuleMessage) -> Task<ModuleMessage> {
        match msg {
            ModuleMessage::TextChanged(input) => self.handle_text_change(input),
            ModuleMessage::WebMessage(inner) => {
                log::trace!("received a webMessage yay!!! inner {inner:?}");

                match inner {
                    WebMsg::GotResult(search_text, res) => {
                        log::trace!("message was result: {res:?}");
                        match res {
                            Ok(o) => {
                                self.cached_results.insert(search_text.clone(), o);
                                // now need to create task for getting images

                                // stupid double clone
                                let client = self.client.clone();
                                let tasks = self
                                    .cached_results
                                    .get(&search_text)
                                    .expect("was just put there, should be fine")
                                    .iter()
                                    .map(|r| Self::handle_getting_image(client.clone(), r.clone()));
                                Task::batch(tasks)
                            }
                            Err(e) => {
                                log::warn!("WebResult was error! {e:?}");
                                Task::none()
                            }
                        }
                    }
                    WebMsg::ResultActivated(url) => {
                        log::info!("Launching webresult with URL: {url}");
                        Self::launch_url(&url);
                        iced::exit()
                    }
                    WebMsg::FetchedImage((url, image)) => {
                        log::trace!(
                            "We got a result for a fetched image! url: {url}, image_handle: {image:?}"
                        );
                        if let Ok(image) = image {
                            self.image_hashmap.insert(url.clone(), image);
                        }
                        Task::none()
                    }
                }
            }
            x => {
                log::trace!("App module received irrelevant msg: {x:?}");
                Task::none()
            }
        }
    }

    fn run(&self) -> Task<crate::message::Message> {
        match self.cached_results.get(&self.input_for_results) {
            Some(v) => {
                if let Some(search_res) = v.first() {
                    Self::launch_url(&search_res.destination_url);
                    iced::exit()
                } else {
                    log::warn!(
                        "Selected search_result list was empty? This doesnt make sense. Self: {self:?}"
                    );
                    Task::none()
                }
            }
            None => Task::none(),
        }
    }
}
