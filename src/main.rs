use iced::{Task, widget};
use pretty_env_logger;

use log;
mod apps;
use apps::App;

#[derive(Clone, Debug)]
enum Message {
    TextInputChanged(String),
    FocusTextInput,
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
        log::info!("update fn run");
        match message {
            Message::TextInputChanged(content) => {
                self.text_value = content;
                if self.app_list.len() == 0 {
                    log::info!("Regenerating app_list");
                    let start = std::time::Instant::now();
                    self.app_list = apps::get_apps();
                    log::info!(
                        "Time to get #{} apps: {:#?}",
                        self.app_list.len(),
                        start.elapsed()
                    )
                }
                Task::none()
            }
            Message::FocusTextInput => widget::text_input::focus(self.text_id.clone()),
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        // the heck is a '_
        log::info!("view fn run");
        let text_input = widget::text_input("placeholder", &self.text_value)
            .id(self.text_id.clone())
            .on_input(Message::TextInputChanged);

        let result = widget::scrollable(widget::column(
            self.app_list
                .clone()
                .into_iter()
                .map(|app| widget::text(app.name).into()),
        ));

        let root_continer = widget::container(widget::column![text_input, result])
            .padding(10)
            .center(iced::Fill);

        root_continer.into()
    }
}

pub fn main() -> iced::Result {
    pretty_env_logger::init();
    log::warn!("aaa");
    iced::application("Lumin", State::update, State::view)
        .subscription(capture_keyboard_input_subscription)
        .level(iced::window::Level::AlwaysOnTop)
        .resizable(false)
        .decorations(false)
        .window_size((800.0, 100.0))
        .theme(|_s| iced::Theme::CatppuccinMocha)
        .run()
}

fn capture_keyboard_input_subscription(_state: &State) -> iced::Subscription<Message> {
    iced::window::open_events().map(|_id| Message::FocusTextInput)
}

// fn window_initialised_subscription(_state: &State) -> iced::Subscription<Message> {
// }
