use std::{any::Any, rc::Rc};

use iced::{Task, widget};

use crate::module::{Module, ModuleMessage};

#[derive(Debug, Clone)]
pub enum WebMsg {
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

    fn update(&mut self, msg: ModuleMessage) -> Task<()> {
        // do nothing (for now)
        // TODO, attempt to downcast.
        //    if let Some(m) = msg.downcast_ref::<CalcMsg>() {
        //
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

                    // Task::perform(future, f)
                }
                Task::none()
            },
        ModuleMessage::WebMessage(_inner) => {
            Task::none()
        },
        _ => Task::none()
        }
    
    }

    fn run(&self) {
        println!("first result is: {:?}", self.cached_results.first())
    }
}
