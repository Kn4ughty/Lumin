use iced::{Task, widget};
use std::io::Write;

use crate::{
    module::{Module, ModuleMessage},
    sorting, widglets,
};

pub struct Drun {
    options: Vec<String>,
    text_input: String,
    selected_index: usize,
}

impl Drun {
    pub fn new(input: Vec<String>) -> Self {
        Drun {
            options: input,
            text_input: "".into(),
            selected_index: 0,
        }
    }

    fn run_at_index(&self, index: usize) {
        let mut stdout = std::io::stdout();

        if self.options.is_empty() {
            stdout.write_all(self.text_input.as_bytes())
        } else {
            stdout.write_all(
                self.options
                    .get(index)
                    .expect("Can get option at requested index")
                    .as_bytes(),
            )
        }
        .expect("Can write to stdoi");
    }
}

impl Module for Drun {
    fn update(&mut self, msg: ModuleMessage) -> iced::Task<ModuleMessage> {
        match msg {
            ModuleMessage::TextChanged(input) => {
                self.selected_index = 0;
                self.options
                    .sort_by_cached_key(|opt| -sorting::score_element(&input, opt));
                Task::none()
            }
            ModuleMessage::ActivatedIndex(i) => {
                self.run_at_index(i);
                Task::none()
            }
            ModuleMessage::SelectionUp => {
                if self.selected_index >= 1 {
                    self.selected_index -= 1
                }
                Task::none()
            }
            ModuleMessage::SelectionDown => {
                if self.selected_index + 1 < self.options.len() {
                    self.selected_index += 1
                }
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
                .map(|(i, item)| {
                    widglets::ListRow::new(item)
                        .on_activate(ModuleMessage::ActivatedIndex(i))
                        .selected(self.selected_index == i)
                        .into()
                }),
        ))
        .into()
    }

    fn run(&self) -> iced::Task<crate::message::Message> {
        self.run_at_index(self.selected_index);
        iced::exit()
    }
}
