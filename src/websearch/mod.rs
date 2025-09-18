use reqwest;
use iced::{Task, widget};

use crate::module::{Module, ModuleMessage};

#[derive(Debug, Clone)]
pub enum WebMsg {
    // GotResult(reqwest::Response),
    GotResult(String),
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
                    // Must be new text
                    self.cached_results.clear();
                    self.input_for_results = input.to_string();

                    for i in 0..=9 {
                        self.cached_results
                            .push(format!("result: {i}. input: {input}\n"))
                    }

                    // Need to tokio
                    return Task::perform(async {let f = reqwest::get("https://example.com");
                        let Ok(j) = f.await else { return "ERROR".to_string()};

                        let Ok(i) = j.text().await else { return "ERORR 2".to_string()};
                        return i.to_string()
                    },
                        |r| ModuleMessage::WebMessage(WebMsg::GotResult(r)))
                }
                Task::none()
            },
        ModuleMessage::WebMessage(inner) => {
            log::warn!("received a webMessage yay!!! inner {inner:?}");
            Task::none()
        },
        _ => Task::none()
        }
    
    }

    fn run(&self) {
        println!("first result is: {:?}", self.cached_results.first())
    }
}
