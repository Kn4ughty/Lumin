#![deny(clippy::unwrap_used)]

use iced::{Task, Widget, keyboard, theme::Style, widget};

use std::path::PathBuf;
use std::cell::LazyCell;
use std::collections::HashMap;

mod apps;

mod calculator;
mod message;

mod websearch;

mod drun;
use drun::Drun;

mod config;
mod constants;
mod module;
mod serworse;
mod sorting;
mod util;
mod widglets;
mod history;
use module::{Module, ModuleEnum, ModuleMessage};

use message::Message;

use crate::constants::DATA_DIR;
use crate::history::History;

const HELP_SCREEN_PREFIX: &str = "?";

struct State {
    text_value: String,
    text_id: widget::Id,
    /// Used for showing the help screen on startup
    has_user_typed: bool,
    window_id: Option<iced::window::Id>,
    modules: HashMap<String, LazyCell<Box<dyn Module>>>,
    module_types: Vec<(String, ModuleEnum)>,
    history_suggestion: Option<String>,
    history: History
}

// Startup things
impl State {
    fn new_multi_modal() -> (Self, Task<Message>) {
        let start = std::time::Instant::now();
        let mut modules: HashMap<String, LazyCell<Box<dyn Module>>> = HashMap::new();

        let mut module_types = Vec::new();

        for (mod_enum, prefix) in config::SETTINGS
            .lock()
            .expect("mutex")
            .app_prefixes
            .iter()
            .filter(|(module, _)| {
                **module != ModuleEnum::HelpScreen && !matches!(**module, ModuleEnum::Unknown(_))
            })
        {
            modules.insert(prefix.to_string(), LazyCell::new(mod_enum.into()));
            module_types.push((prefix.clone(), mod_enum.clone()));
        }

        log::info!("Time to initialise modules: {:#?}", start.elapsed());

        (
            State {
                text_value: String::new(),
                text_id: widget::Id::new("text_entry"),
                window_id: None,
                has_user_typed: false,
                modules,
                module_types,
                history_suggestion: None,
                history: History::load(),
            },
            Self::load_font(),
        )
    }

    fn new_drun() -> (Self, Task<Message>) {
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

        (
            State {
                text_value: String::new(),
                text_id: widget::Id::new("text_entry"),
                window_id: None,
                has_user_typed: false,
                modules,
                module_types: Vec::new(),
                history_suggestion: None,
                history: History::load()
            },
            Self::load_font(),
        )
    }

    fn load_font() -> Task<Message> {
        iced::font::load(include_bytes!("../assets/lexend/LexendDeca-Regular.ttf"))
            .map(Message::FontLoaded)
    }
}

impl State {
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
                self.history.add_query(self.text_value.clone());
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
                if let Some(history_suggestion) = self.history.get_above(self.text_value.clone()) {
                    self.history_suggestion = Some(history_suggestion)
                }
                log::info!("Dummy Text: {:?}", self.history_suggestion);

                if let Some((module, _)) = self.find_module_mut() {
                    return module
                        .update(ModuleMessage::SelectionUp)
                        .map(Message::PluginMessage);
                }

                Task::none()
            }
            Message::KeyboardDown => {
                // We don't want to go to nothing when we hit the top of the list, but when we go back down the final item **should** be empty, so no .or()
                self.history_suggestion = self.history.get_below(self.text_value.clone());
                log::info!("Dummy Text: {:?}", self.history_suggestion);

                if let Some((module, _)) = self.find_module_mut() {
                    return module
                        .update(ModuleMessage::SelectionDown)
                        .map(Message::PluginMessage);
                }
                Task::none()
            }
            Message::FillAccepted => {
                log::info!("Filling suggestion");
                self.text_value = self.history_suggestion.clone().unwrap_or_default();
                self.history_suggestion = None;
                Task::batch(
                    [
                        Task::done(Message::TextInputChanged(self.text_value.clone())),
                        widget::operation::text_input::move_cursor_to_end(self.text_id.clone())
                    ]
                )
            }
            Message::FontLoaded(res) => {
                if let Err(e) = res {
                    log::error!("Could not load font: {e:#?}");
                }
                log::debug!("Loaded font successfully");
                Task::none()
            }
            Message::Unfocused => {
                if config::SETTINGS.lock().expect("mutex").close_on_unfocus {
                    log::info!("Lost focus; app is exiting.");
                    iced::exit()
                } else {
                    Task::none()
                }
            }
            Message::Clipboard(t) => {
                if let Err(e) = t {
                    log::error!("Error copying to clipboard: {e:?}")
                }
                Task::none()
            }
            Message::DoNothing => Task::none(),
        }
    }

    fn view(&self) -> impl iced::Widget<Message> {
        log::trace!("view fn run");

        let input_prompt = self.history_suggestion.clone().unwrap_or(
            if self.text_value.is_empty() {
                config::SETTINGS.lock().expect("mutex").input_prompt.clone()
            } else {
                String::new()
            }
        );

        let text_input = widglets::text_input_2::TextInput::new(input_prompt, &self.text_value)
            .id(self.text_id.clone())
            .on_input(Message::TextInputChanged)
            .on_submit(Message::TextInputSubmitted("test".to_string()))
            .padding(8.0)
            .style(|theme, status| {
                let mut base_style = widglets::text_input_2::default(theme, status);
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
            if config::SETTINGS
                .lock()
                .expect("mutex")
                .transparent_background
            {
                base_theme = base_theme.background(iced::Color::TRANSPARENT);
            } else {
                base_theme = base_theme.background(theme.palette().background.base.color)
            }
            base_theme
        })
        .padding(10)
        .align_top(iced::Fill);

        let mut mouse = widget::mouse_area(root_continer).on_press(Message::ShouldDrag);
        // Hide the mouse if the user has not typed yet. Looks better.
        if !self.has_user_typed {
            mouse = mouse.interaction(iced::mouse::Interaction::Hidden);
        }

        mouse
    }

    fn style(&self, theme: &iced::Theme) -> Style {
        Style {
            background_color: iced::Color::TRANSPARENT,
            text_color: theme.palette().primary.base.text,
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
            .boxed()
    }

    fn show_overview_screen(&self) -> iced::Element<'_, Message> {
        let mut prefix_col = widget::column![widget::text("Prefix")];
        prefix_col = prefix_col.push(widget::rule::horizontal(1).boxed());

        let mut description_col = widget::column![widget::text("Description")];
        description_col = description_col.push(widget::rule::horizontal(1).boxed());

        let mut all_modules: Vec<(String, String)> = self
            .module_types
            .iter()
            .map(|(prefix, module)| (prefix.clone(), module.description()))
            .collect();

        all_modules.sort_by(|first, other| first.0.cmp(&other.0));

        // Since the help_screen module is magic, it needs special logic
        all_modules.push((
            HELP_SCREEN_PREFIX.to_string(),
            "This help screen".to_string(),
        ));

        for (prefix, module) in all_modules {
            prefix_col = prefix_col.push(widget::text(prefix).boxed());
            prefix_col = prefix_col.push(widget::rule::horizontal(1).boxed());

            description_col = description_col.push(widget::text(module).boxed());
            description_col = description_col.push(widget::rule::horizontal(1).boxed());
        }

        widget::row![
            widget::container(prefix_col).width(iced::Shrink),
            widget::container(widget::rule::vertical(2)).padding(8),
            description_col
        ]
        .boxed()
    }

    fn theme(&self) -> Option<iced::Theme> {
        Some(config::SETTINGS.lock().expect("mutex").clone().color_scheme)
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
        iced::window::events().map(|event| {
            if event.1 == iced::window::Event::Unfocused {
                Message::Unfocused
            } else {
                Message::DoNothing
            }
        }),
        // Thank you https://kressle.in/keystrokes
        // iced::keyboard::on_key_press(handle_press_hotkeys),
        // iced::keyboard::on_key_release(handle_release_hotkeys),
        iced::keyboard::listen().map(|ke| {
            match ke {
                keyboard::Event::KeyPressed { key, modifiers, .. } => {
                    handle_press_hotkeys(key, modifiers)
                }
                keyboard::Event::KeyReleased { key, modifiers, .. } => {
                    handle_release_hotkeys(key, modifiers)
                }
                keyboard::Event::ModifiersChanged(_) => None,
            }
            .unwrap_or(Message::DoNothing)
        }), // Todo, work out how to subscribe to mouse movement
            // https://docs.iced.rs/iced/mouse/index.html
    ])
}

fn handle_press_hotkeys(key: keyboard::Key, modifier: keyboard::Modifiers) -> Option<Message> {
    use iced::keyboard as kb;
    use iced::keyboard::Modifiers as kmod;

    match (key.as_ref(), modifier) {
        (kb::Key::Named(kb::key::Named::Escape), _) => Some(Message::Close),
        (kb::Key::Named(kb::key::Named::ArrowUp), _) => Some(Message::KeyboardUp),
        (kb::Key::Named(kb::key::Named::ArrowDown), _) => Some(Message::KeyboardDown),
        (kb::Key::Named(kb::key::Named::Tab), _) => Some(Message::FillAccepted),
        _ => None,
    }
}

fn handle_release_hotkeys(key: keyboard::Key, _modifier: keyboard::Modifiers) -> Option<Message> {
    use iced::keyboard as kb;

    match (key.as_ref(), _modifier) {
        // This is a bit silly
        (kb::Key::Named(kb::key::Named::Escape), _) => Some(Message::Close),
        _ => None,
    }
}

fn main() -> Result<(), String> {
    pretty_env_logger::init();

    let mut state: fn() -> (State, Task<Message>) = State::new_multi_modal;

    // Skip first arg (program name)
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
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
                state = State::new_drun;
            }
            "-p" => {
                let Some(prompt) = args.next() else {
                    return Err("Missing prompt name after -p argument".to_string());
                };
                // Safety. The program has not started yet, so there cannot be anything else writing
                // to it at this point of execution
                config::SETTINGS.lock().expect("mutex").input_prompt = prompt;
            }
            "--no_icon" => {
                config::SETTINGS.lock().expect("mutex").show_icons = false;
            }
            unknown => log::warn!("Unknown arg {unknown}"),
        }
    }

    iced::application(state, State::update, State::view)
        .title("Lumin")
        .settings(iced::Settings {
            font: iced::Font {
                family: iced::font::Family::Name("Lexend Deca"),
                ..Default::default()
            },
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
        .map_err(|e| format!("Iced Error: {e:#?}"))
}
