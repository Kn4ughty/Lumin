use iced::{Task, widget};
use std::collections::HashMap;

use crate::{
    module::{Module, ModuleMessage},
    util,
    websearch::bits::{SearchError, WebImage},
    widglets,
};

mod bits;
use bits::SearchResult;
pub use bits::WebMsg;
mod wikipedia;

pub struct Web {
    input_for_results: String,
    cached_results: Vec<SearchResult>, // TODO. Convert to hashmap with input for actual caching
    // Memory, who needs it anyway
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
            cached_results: vec![],
            image_hashmap: HashMap::new(),
            client: reqwest::ClientBuilder::new()
                // https://foundation.wikimedia.org/wiki/Special:MyLanguage/Policy:User-Agent_policy
                .user_agent("LuminAppLauncher/0.0 (https://github.com/Kn4ughty)")
                .build()
                .unwrap(),
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
                let client = self.client.clone();
                Task::perform(
                    async move { wikipedia::search(&client, &search_text[1..]).await },
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

    fn handle_getting_image(client: reqwest::Client, input: SearchResult) -> Task<ModuleMessage> {
        log::trace!("handle_getting_image ran. SR: {input:?}");

        let Some(url) = input.image else {
            return Task::none();
        };
        let url: Option<String> = match url {
            WebImage::URL(s) => Some(s),
            WebImage::ImageData(_) => {
                // TODO. make invalid states unrepresentable
                log::warn!("Image data got into handle_getting_images.");
                None
            }
        };
        let Some(url) = url else { return Task::none() };

        // => Result<ModuleMessage, SearchError>

        Task::perform(
            async move { (url.clone(), Self::get_image(client, &url).await) },
            |r| {
                ModuleMessage::WebMessage(WebMsg::FetchedImage((
                    r.0,
                    widget::image::Handle::from_bytes(r.1.unwrap()),
                )))
            },
        )
    }

    async fn get_image(
        client: reqwest::Client,
        url: &str,
    ) -> Result<iced::advanced::image::Bytes, SearchError> {
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| SearchError::BadResponse(format!("failed to get response: {}", e)))?;
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
    fn view(&self) -> iced::Element<'_, ModuleMessage> {
        log::trace!("Web view function run");
        let elements: Vec<iced::Element<'_, ModuleMessage>> = self
            .cached_results
            .clone()
            .into_iter()
            .map(|result| {
                let img = match result.image {
                    Some(WebImage::URL(s)) => {
                        log::trace!("web image was url: {s}");
                        None
                    }
                    Some(WebImage::ImageData(d)) => Some(d),
                    None => None,
                };

                widglets::listrow(
                    result.title,
                    Some(result.description),
                    Some(ModuleMessage::WebMessage(WebMsg::ResultSelected(
                        result.url,
                    ))), // eww
                    img,
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
                            Ok(o) => {
                                self.cached_results = o;
                                // now need to create task for getting images

                                // stupid double clone
                                let client = self.client.clone();
                                let tasks = self
                                    .cached_results
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
                    WebMsg::ResultSelected(url) => {
                        log::info!("Launching webresult with URL: {url}");
                        Self::launch_url(&url);
                        iced::exit()
                    }
                    WebMsg::FetchedImage((url, image)) => {
                        // let image = iced::advanced::image::Image::new(
                        //     iced::advanced::image::Handle::from_bytes(image),
                        // );
                        // self.image_hashmap.insert(url.clone(), image);

                        for res in self.cached_results.iter_mut() {
                            match &res.image {
                                Some(WebImage::URL(u)) => {
                                    if *u == url {
                                        res.image = Some(WebImage::ImageData(image));
                                        return Task::none();
                                    }
                                }
                                _ => return Task::none(),
                            }
                            // if res.image == url {
                            //     res.image = Some(WebImage::ImageData(image));
                            //     return Task::none()
                            // }
                        }
                        log::warn!("url did not match any result image: {url}");
                        return Task::none();
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
