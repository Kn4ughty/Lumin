#![deny(clippy::unwrap_used)]

use iced::{Task, keyboard, theme::Style, widget};

use std::cell::LazyCell;
use std::collections::HashMap;

mod apps;
use apps::AppModule;

mod calculator;
mod message;
use calculator::Calc;

mod websearch;
use websearch::Web;

mod drun;
use drun::Drun;

mod files;
use files::FileSearcher;

mod config;
mod constants;
mod module;
mod serworse;
mod sorting;
mod util;
mod widglets;
use module::{Module, ModuleMessage};

use message::Message;

const HELP_SCREEN_PREFIX: &str = "?";

struct State {
    text_value: String,
    text_id: widget::Id,
    /// Used for showing the help screen on startup
    has_user_typed: bool,
    window_id: Option<iced::window::Id>,
    modules: HashMap<String, LazyCell<Box<dyn Module>>>,
}

impl State {
    fn new_multi_modal() -> Self {
        let start = std::time::Instant::now();
        let mut modules: HashMap<String, LazyCell<Box<dyn Module>>> = HashMap::new();
        modules.insert("=".to_string(), LazyCell::new(|| Box::new(Calc::new())));

        modules.insert("!".to_string(), LazyCell::new(|| Box::new(Web::new())));

        modules.insert(
            "'".to_string(),
            LazyCell::new(|| Box::new(FileSearcher::new())),
        );

        modules.insert(String::new(), LazyCell::new(|| Box::new(AppModule::new())));

        log::info!("Time to initialise modules: {:#?}", start.elapsed());
        State {
            text_value: String::new(),
            text_id: widget::Id::new("text_entry"),
            window_id: None,
            has_user_typed: false,
            modules,
        }
    }

    fn new_drun() -> Self {
        let start = iced::debug::time("load modules");
        let mut modules: HashMap<String, LazyCell<Box<dyn Module>>> = HashMap::new();

        modules.insert(
            String::new(),
            LazyCell::new(|| {
                let stdin = std::io::stdin();
                let mut lines = Vec::new();
                for line in stdin.lines() {
                    match line {
                        Ok(line_ok) => lines.push(line_ok),
                        Err(e) => log::warn!("Line was not valid utf8!!: {e:?}"),
                    }
                }
                Box::new(Drun::new(lines))
            }),
        );

        start.finish();

        State {
            text_value: String::new(),
            text_id: widget::Id::new("text_entry"),
            window_id: None,
            has_user_typed: false,
            modules,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        log::trace!("update fn run");

        match message {
            Message::TextInputChanged(content) => {
                self.has_user_typed = true;
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

                self.find_module().expect("Can find module").0.run()
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
                log::trace!("Handling module message {a:?}");
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
            Message::KeyboardUp => {
                if let Some((module, _)) = self.find_module_mut() {
                    return module
                        .update(ModuleMessage::SelectionUp)
                        .map(Message::PluginMessage);
                }
                Task::none()
            }
            Message::KeyboardDown => {
                if let Some((module, _)) = self.find_module_mut() {
                    return module
                        .update(ModuleMessage::SelectionDown)
                        .map(Message::PluginMessage);
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        log::trace!("view fn run");

        let text_input = widget::text_input("Type to search", &self.text_value)
            .id(self.text_id.clone())
            .on_input(Message::TextInputChanged)
            .on_submit(Message::TextInputSubmitted("test".to_string()))
            .padding(8.0)
            .style(|theme, status| {
                let mut base_style = widget::text_input::default(theme, status);
                base_style.border = iced::Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: 10.0.into(),
                };
                base_style
            });

        let result = self.get_result_to_display();

        let root_continer = widget::container(widget::column![
            text_input,
            widget::space().height(8),
            result
        ])
        .style(|theme| {
            let mut base_theme = widget::container::bordered_box(theme);
            base_theme.border = iced::Border {
                radius: 15.0.into(),
                ..base_theme.border
            };
            if config::SETTINGS.transparent_background {
                base_theme = base_theme.background(iced::Color::TRANSPARENT);
            }
            base_theme
        })
        .padding(10)
        .align_top(iced::Fill);

        let mouse = widget::mouse_area(root_continer).on_press(Message::ShouldDrag);

        mouse.into()
    }

    fn style(&self, theme: &iced::Theme) -> Style {
        Style {
            background_color: iced::Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }

    fn get_result_to_display(&self) -> iced::Element<'_, Message> {
        if (!self.has_user_typed || self.text_value == HELP_SCREEN_PREFIX)
            && self.modules.len() != 1
        {
            return self.show_overview_screen();
        }

        self.find_module()
            .expect("can find module")
            .0
            .view()
            .map(|s: ModuleMessage| Message::PluginMessage(s))
    }

    fn show_overview_screen(&self) -> iced::Element<'_, Message> {
        let mut prefix_col = widget::column![widget::text("Prefix")];
        prefix_col = prefix_col.push(widget::rule::horizontal(1));

        let mut description_col = widget::column![widget::text("Description")];
        description_col = description_col.push(widget::rule::horizontal(1));

        let mut all_modules: Vec<(String, String)> = self
            .modules
            .iter()
            .map(|(key, module)| (key.clone(), module.description()))
            .collect();
        all_modules.sort_by(|first, other| first.0.cmp(&other.0));

        // Since the help_screen module is magic, it needs special logic
        all_modules.push((
            HELP_SCREEN_PREFIX.to_string(),
            "This help screen".to_string(),
        ));

        for (prefix, module) in all_modules {
            prefix_col = prefix_col.push(widget::text(prefix));
            prefix_col = prefix_col.push(widget::rule::horizontal(1));

            description_col = description_col.push(widget::text(module));
            description_col = description_col.push(widget::rule::horizontal(1));
        }

        widget::row![
            widget::container(prefix_col).width(iced::Shrink),
            widget::container(widget::rule::vertical(2)).padding(8),
            description_col
        ]
        .into()
    }

    fn theme(&self) -> Option<iced::Theme> {
        Some(config::SETTINGS.clone().color_scheme)
    }

    #[allow(clippy::borrowed_box)]
    fn find_module(&self) -> Option<(&LazyCell<Box<dyn Module>>, usize)> {
        self.modules
            .iter()
            .filter(|(k, _)| self.text_value.starts_with(k.as_str()))
            .max_by_key(|(prefix, _)| prefix.len())
            .map(|(prefix, m)| (m, prefix.len()))
    }

    fn find_module_mut(&mut self) -> Option<(&mut LazyCell<Box<dyn Module>>, usize)> {
        let start = iced::debug::time("find_module_mut");
        let res = self
            .modules
            .iter_mut()
            .filter(|(k, _)| self.text_value.starts_with(k.as_str()))
            .max_by_key(|(prefix, _)| prefix.len())
            .map(|(prefix, m)| (m, prefix.len()));
        start.finish();
        res
    }
}

fn subscription(_state: &State) -> iced::Subscription<Message> {
    iced::Subscription::batch(vec![
        iced::window::open_events().map(Message::WindowOpened),
        iced::keyboard::on_key_release(handle_hotkeys),
    ])
}

// Thank you https://kressle.in/keystrokes
fn handle_hotkeys(key: keyboard::Key, modifier: keyboard::Modifiers) -> Option<Message> {
    use iced::keyboard as kb;
    use iced::keyboard::Modifiers as kmod;

    match (key.as_ref(), modifier) {
        // This is a bit silly
        (kb::Key::Named(kb::key::Named::Escape), _) => Some(Message::Close),
        (kb::Key::Named(kb::key::Named::ArrowUp), _) => Some(Message::KeyboardUp),
        (kb::Key::Named(kb::key::Named::ArrowDown), _) => Some(Message::KeyboardDown),
        (kb::Key::Named(kb::key::Named::Tab), kmod::SHIFT) => Some(Message::KeyboardUp),
        (kb::Key::Named(kb::key::Named::Tab), _) => Some(Message::KeyboardDown),
        _ => None,
    }
}

fn main() -> iced::Result {
    pretty_env_logger::init();

    let mut state = State::new_multi_modal as fn() -> State;

    // Skip first arg (program name)
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-v" | "--version" => {
                println!(
                    "{} Version: {}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"),
                );
                return Ok(());
            }
            "--dmenu" => {
                state = State::new_drun as fn() -> State;
            }
            unknown => log::warn!("Unknown arg {unknown}"),
        }
    }

    iced::application(state, State::update, State::view)
        .title("Lumin")
        .settings(iced::Settings {
            default_font: iced::Font::MONOSPACE,
            ..Default::default()
        })
        .subscription(subscription)
        .level(iced::window::Level::AlwaysOnTop)
        .antialiasing(true)
        .window(iced::window::Settings {
            blur: true,
            resizable: false,
            decorations: false,
            transparent: true,
            size: (800.0, 330.0).into(),

            ..Default::default()
        })
        .theme(State::theme)
        .style(State::style)
        .run()
}
