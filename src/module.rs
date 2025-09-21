use iced::{Element, Task};

// use crate::calculator;
// use crate::apps;
use crate::websearch;

#[derive(Debug, Clone)]
pub enum ModuleMessage {
    TextChanged(String),
    WebMessage(websearch::WebMsg),
}

pub trait Module {
    fn update(&mut self, msg: ModuleMessage) -> Task<ModuleMessage>;
    fn view(&self) -> Element<'_, ModuleMessage>; // why is string here?
    fn run(&self); // Allow to return result
}

// use std::any::Any;
// use std::rc::Rc;
//
// use iced::{Element, Task};
//
// pub type ModulePayload = Rc<dyn Any>;
//
// pub trait Module {
//     fn on_input(&mut self, text: &str) -> Task<()>;
//     fn update(&mut self, msg: ModulePayload) -> Task<()>;
//     fn view(&self) -> Element<'_, AppMessage>;
//     fn run(&self); // Allow to return result
// }
//
