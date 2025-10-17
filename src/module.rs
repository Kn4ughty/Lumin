use iced::{Element, Task};

use crate::websearch;

#[derive(Debug, Clone)]
pub enum ModuleMessage {
    TextChanged(String),
    ActivatedIndex(usize),
    WebMessage(websearch::WebMsg),
}

pub trait Module {
    fn update(&mut self, msg: ModuleMessage) -> Task<ModuleMessage>;
    fn view(&self) -> Element<'_, ModuleMessage>;
    fn run(&self); // Allow to return result
}
