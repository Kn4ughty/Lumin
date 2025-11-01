use iced::{Task, widget};
use std::io::Write;

use crate::{
    module::{Module, ModuleMessage},
    util, widglets,
};

pub struct Drun {
    options: Vec<String>,
}

impl Drun {
    pub fn new(input: Vec<String>) -> Self {
        Drun { options: input }
    }

    fn run_at_index(&self, index: usize) {
        let mut stdout = std::io::stdout();
        stdout
            .write_all(
                self.options
                    .get(index)
                    .expect("Can get option at requested index")
                    .as_bytes(),
            )
            .expect("Can write to stdout");
    }
}

impl Module for Drun {
    fn update(&mut self, msg: ModuleMessage) -> iced::Task<ModuleMessage> {
        match msg {
            ModuleMessage::TextChanged(input) => {
                let input = &input.to_lowercase();
                self.options.sort_by_cached_key(|opt| {
                    // Not a fan of this duplicated logic from app/mod.rs
                    let mut score = util::longest_common_substr(opt, input);
                    if opt.to_lowercase().starts_with(input) {
                        score += 2;
                    }
                    -score
                });
                Task::none()
            }
            ModuleMessage::ActivatedIndex(i) => {
                self.run_at_index(i);
                Task::none()
            }
            _ => {
                log::warn!("unknown message!");
                Task::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, ModuleMessage> {
        widget::scrollable(widget::column(
            self.options
                .clone()
                .into_iter()
                .enumerate()
                .map(|(i, opt)| {
                    widglets::listrow(opt, None, Some(ModuleMessage::ActivatedIndex(i)), None)
                        .into()
                }),
        ))
        .into()
    }

    fn run(&self) -> iced::Task<crate::message::Message> {
        self.run_at_index(0);
        iced::exit()
    }
}
