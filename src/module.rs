use iced::Element;

pub trait Module {
    fn update(&mut self, input: &str);
    fn view(&self) -> Element<'_, String>; // why is string here?
    fn run(&self); // Allow to return result
}

// use std::any::Any;
// use std::rc::Rc;
//
// use iced::{Element, Task};
//
// pub type ModulePayload = Rc<dyn Any>;
// #[derive(Debug, Clone)]
// pub enum AppMessage {
//     TextChanged(String),            // always emitted on input change
//     ModuleMessage { prefix: String, payload: ModulePayload },
//     Close,
// }
//
// pub trait Module {
//     fn on_input(&mut self, text: &str) -> Task<()>;
//     fn update(&mut self, msg: ModulePayload) -> Task<()>;
//     fn view(&self) -> Element<'_, AppMessage>;
//     fn run(&self); // Allow to return result
// }
//
