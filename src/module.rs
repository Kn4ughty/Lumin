use iced::{Element, Task};

use crate::apps;
use crate::message::Message;
use crate::websearch;

#[derive(Debug, Clone)]
pub enum ModuleMessage {
    TextChanged(String),
    ActivatedIndex(usize),
    AppMessage(apps::AppMessage),
    WebMessage(websearch::WebMsg),
    DoNothing,
}

pub trait Module {
    fn update(&mut self, msg: ModuleMessage) -> Task<ModuleMessage>;
    fn view(&self) -> Element<'_, ModuleMessage>;
    /// Executed when user presses the enter key
    fn run(&self) -> Task<Message>;
    /// Short overview of what the module does
    fn description(&self) -> String {
        String::from("No description")
    }
}
