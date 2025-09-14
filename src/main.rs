use std::collections::HashMap;

use iced::{Task, keyboard, widget};
use pretty_env_logger;

use log;

mod apps;
use apps::AppModule;

mod calculator;
use calculator::Calc;

mod module;
mod widglets;
mod util;
use module::Module;

#[derive(Clone, Debug)]
enum Message {
    TextInputChanged(String),
    FocusTextInput,
    TextInputSubmitted(String),
    Close,
    PluginMessage(String),
}

struct State {
    text_value: String,
    text_id: widget::text_input::Id,
    modules: HashMap<String, Box<dyn Module>>,
}

impl std::default::Default for State {
    fn default() -> State {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
        modules.insert(";c".to_string(), Box::new(Calc::new()));
        modules.insert("".to_string(), Box::new(AppModule::new()));

        State {
            text_value: "".to_string(),
            text_id: widget::text_input::Id::new("text_entry"),
            modules,
        }
    }
}

impl State {
    fn update(&mut self, message: Message) -> Task<Message> {
        log::trace!("update fn run");
        match message {
            Message::TextInputChanged(content) => {
                self.text_value = content;
                // Lookup module and pass in text
                let input = self.text_value.clone();
                if let Some((module, prefix_size)) = self.find_module_mut() {
                    module.update(&input[prefix_size..]);
                }

                Task::none()
            }
            Message::TextInputSubmitted(_text) => {
                log::info!("Text input submitted");

                // TODO. Dont just unwrap
                self.find_module().unwrap().0.run();
                iced::exit()
            }
            Message::FocusTextInput => widget::text_input::focus(self.text_id.clone()),
            Message::Close => {
                log::info!("App is exiting");
                iced::exit()
            }
            Message::PluginMessage(a) => {
                log::info!("Ignoring plugin message {a}");
                Task::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        // the heck is a '_
        log::trace!("view fn run");
        let text_input = widget::text_input("Type to search", &self.text_value)
            .id(self.text_id.clone())
            .on_input(Message::TextInputChanged)
            .on_submit(Message::TextInputSubmitted("test".to_string()));

        let result = self
            .find_module()
            .unwrap()
            .0
            .view()
            .map(|s: String| Message::PluginMessage(s));

        let root_continer = widget::container(widget::column![text_input, result])
            .padding(10)
            .align_top(iced::Fill);

        root_continer.into()
    }

    fn find_module(&self) -> Option<(&Box<dyn Module>, usize)> {
        self.modules
            .iter()
            .filter(|(k, _)| self.text_value.starts_with(k.as_str()))
            .max_by_key(|(prefix, _)| prefix.len())
            .map(|(prefix, m)| (m, prefix.len()))
            // .find(|(prefix, _mod)| self.text_value.starts_with(prefix.as_str()))
    }

    fn find_module_mut(&mut self) -> Option<(&mut Box<dyn Module>, usize)> {
        self.modules
            .iter_mut()
            .filter(|(k, _)| self.text_value.starts_with(k.as_str()))
            .max_by_key(|(prefix, _)| prefix.len())
            .map(|(prefix, m)| (m, prefix.len()))
            // .find(|(prefix, _mod)| self.text_value.starts_with(prefix.as_str()))
            // .map(|(_s, m)| m)
    }
}

pub fn main() -> iced::Result {
    #[cfg(feature = "perf")] {
        let start = std::time::Instant::now();

        apps::get_apps();
        println!("Time to get apps: {:#?}", start.elapsed());
        return Ok(());
    }

    pretty_env_logger::init();
    iced::application("Lumin", State::update, State::view)
        .subscription(subscription)
        .level(iced::window::Level::AlwaysOnTop)
        .resizable(false)
        .decorations(false)
        .window_size((800.0, 200.0))
        .theme(|_s| {
            let theme = iced::Theme::custom(
                "name".to_string(),
                iced::theme::Palette {
                    background: iced::color!(0x313244),
                    ..iced::Theme::CatppuccinMocha.palette()
                },
            );
            theme
        })
        .run()
}

fn subscription(_state: &State) -> iced::Subscription<Message> {
    iced::Subscription::batch(vec![
        iced::window::open_events().map(|_id| Message::FocusTextInput),
        iced::keyboard::on_key_release(handle_hotkeys),
    ])
}

// Thank you https://kressle.in/keystrokes
fn handle_hotkeys(key: keyboard::Key, _modifier: keyboard::Modifiers) -> Option<Message> {
    match key.as_ref() {
        // This is a bit silly
        keyboard::Key::Named(keyboard::key::Named::Escape) => Some(Message::Close),
        _ => None,
    }
}
