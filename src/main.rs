use iced::{Task, keyboard, widget};
use pretty_env_logger;

use log;
mod apps;
use apps::App;
mod util;

#[derive(Clone, Debug)]
enum Message {
    TextInputChanged(String),
    FocusTextInput,
    TextInputSubmitted(String),
    Close,
}

struct State {
    text_value: String,
    text_id: widget::text_input::Id,
    app_list: Vec<App>,
}

impl std::default::Default for State {
    fn default() -> State {
        State {
            text_value: "".to_string(),
            text_id: widget::text_input::Id::new("text_entry"),
            app_list: Vec::new(),
        }
    }
}

impl State {
    fn update(&mut self, message: Message) -> Task<Message> {
        log::trace!("update fn run");
        match message {
            Message::TextInputChanged(content) => {
                self.text_value = content;

                if self.app_list.len() == 0 {
                    log::trace!("Regenerating app_list");
                    let start = std::time::Instant::now();
                    self.app_list = apps::get_apps();
                    log::debug!(
                        "Time to get #{} apps: {:#?}",
                        self.app_list.len(),
                        start.elapsed()
                    )
                }

                let start = std::time::Instant::now();
                // Cached_key seems to be much faster which is interesting since text_value is
                // always changing
                self.app_list.sort_by_cached_key(|app| {
                    let score = util::longest_common_substr(&app.name, &self.text_value);
                    // TODO. Add aditional weighting for first character matching
                    return score * -1;
                });

                log::debug!(
                    "Time to sort #{} apps: {:#?}",
                    self.app_list.len(),
                    start.elapsed()
                );

                Task::none()
            }
            Message::TextInputSubmitted(_text) => {
                log::info!("Text input submitted");
                // TODO. Dont just unwrap
                // Getting into this situation seems unlikely
                self.app_list.first().unwrap().execute().unwrap();
                iced::exit()
            }
            Message::FocusTextInput => widget::text_input::focus(self.text_id.clone()),
            Message::Close => {
                log::info!("App is exiting");
                iced::exit()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        // the heck is a '_
        log::trace!("view fn run");
        let text_input = widget::text_input("placeholder", &self.text_value)
            .id(self.text_id.clone())
            .on_input(Message::TextInputChanged)
            .on_submit(Message::TextInputSubmitted("test".to_string()));

        let result = match self.text_value {
            // where different search modes will go
            _ => widget::scrollable(
                widget::column(
                    self.app_list
                        .clone()
                        .into_iter()
                        .map(|app| widget::text(app.name).into()),
                )
                .width(iced::Fill),
            ),
        };

        let root_continer = widget::container(widget::column![text_input, result])
            .padding(10)
            .align_top(iced::Fill);

        root_continer.into()
    }
}

pub fn main() -> iced::Result {
    pretty_env_logger::init();
    iced::application("Lumin", State::update, State::view)
        .subscription(subscription)
        .level(iced::window::Level::AlwaysOnTop)
        .resizable(false)
        .decorations(false)
        .window_size((800.0, 200.0))
        .theme(|_s| iced::Theme::CatppuccinMocha)
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
