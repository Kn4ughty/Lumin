use iced::{Task, keyboard, widget};

use std::collections::HashMap;

mod apps;
use apps::AppModule;

mod calculator;
mod message;
use calculator::Calc;

mod websearch;
use websearch::Web;

mod constants;
mod module;
mod serworse;
mod util;
mod widglets;
use module::{Module, ModuleMessage};

use message::Message;

struct State {
    text_value: String,
    text_id: widget::Id,
    window_id: Option<iced::window::Id>,
    modules: HashMap<String, Box<dyn Module>>,
}

impl std::default::Default for State {
    fn default() -> State {
        let start = iced::debug::time("load modules");
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
        modules.insert("=".to_string(), Box::new(Calc::new()));

        modules.insert("!".to_string(), Box::new(Web::new()));

        modules.insert("".to_string(), Box::new(AppModule::new()));

        start.finish();
        State {
            text_value: "".to_string(),
            text_id: widget::Id::new("text_entry"),
            window_id: None,
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
                    return module
                        .update(ModuleMessage::TextChanged(input[prefix_size..].to_string()))
                        .map(Message::PluginMessage);
                }

                Task::none()
            }
            Message::TextInputSubmitted(_text) => {
                log::info!("Text input submitted");

                // TODO. Dont just unwrap
                self.find_module().unwrap().0.run()
            }
            Message::WindowOpened(id) => {
                self.window_id = Some(id);
                widget::operation::focus(self.text_id.clone())
            }
            Message::Close => {
                log::info!("App is exiting");
                iced::exit()
            }
            Message::PluginMessage(a) => {
                log::info!("Handling module message {a:?}");
                // TODO. match by exact prefix and pass message
                if let Some((module, prefix)) = self.find_module_mut() {
                    log::trace!("Module handled had prefix {prefix}");
                    return module.update(a).map(Message::PluginMessage);
                }
                Task::none()
            }
            Message::ShouldDrag => {
                if let Some(id) = self.window_id {
                    log::trace!("Dragging the window");
                    iced::window::drag(id)
                } else {
                    Task::none()
                }
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
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
            .map(|s: ModuleMessage| Message::PluginMessage(s));

        let root_continer = widget::container(widget::column![text_input, result])
            .style(widget::container::bordered_box)
            .padding(10)
            .align_top(iced::Fill);

        let mouse = widget::mouse_area(root_continer).on_press(Message::ShouldDrag);

        mouse.into()
    }

    fn theme(&self) -> Option<iced::Theme> {
        Some(iced::theme::Theme::CatppuccinMocha)
    }

    fn find_module(&self) -> Option<(&Box<dyn Module>, usize)> {
        self.modules
            .iter()
            .filter(|(k, _)| self.text_value.starts_with(k.as_str()))
            .max_by_key(|(prefix, _)| prefix.len())
            .map(|(prefix, m)| (m, prefix.len()))
    }

    fn find_module_mut(&mut self) -> Option<(&mut Box<dyn Module>, usize)> {
        self.modules
            .iter_mut()
            .filter(|(k, _)| self.text_value.starts_with(k.as_str()))
            .max_by_key(|(prefix, _)| prefix.len())
            .map(|(prefix, m)| (m, prefix.len()))
    }
}

fn subscription(_state: &State) -> iced::Subscription<Message> {
    iced::Subscription::batch(vec![
        iced::window::open_events().map(|id| Message::WindowOpened(id)),
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

fn main() -> iced::Result {
    pretty_env_logger::init();

    // Ensure that DATA_DIR exists to stop problems later
    let home = std::env::var("HOME").unwrap();
    let path_str = &(home + constants::DATA_DIR);
    let path = std::path::Path::new(path_str);

    if !std::fs::exists(path).expect("Can check if DATA_DIR exists") {
        std::fs::create_dir_all(path).expect("Could create DATA_DIR");
    }

    iced::application(State::default, State::update, State::view)
        .title("Lumin")
        .settings(iced::Settings {
            default_font: iced::Font::MONOSPACE,
            ..Default::default()
        })
        .subscription(subscription)
        .level(iced::window::Level::AlwaysOnTop)
        .resizable(false)
        .decorations(false)
        .antialiasing(true)
        .transparent(true)
        .window_size((800.0, 300.0))
        .theme(State::theme)
        // .theme(|s| iced::theme::Theme::CatppuccinMocha)
        // .theme(|_s| {
        //     // iced::Theme::custom(
        //     //     // std::borrow::Cow::Borrowed("name"),
        //     //     // iced::theme::Palette {
        //     //     //     background: iced::color!(0x313244),
        //     //     //     ..iced::Theme::CatppuccinMocha.palette()
        //     //     // },
        //     // )
        //     iced::Theme::CatppuccinMocha
        // })
        .run()
}
