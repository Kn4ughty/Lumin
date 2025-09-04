use iced::{Task, widget};
mod log;

#[derive(Clone, Debug)]
enum Message {
    TextInputChanged(String),
    FocusTextInput,
}

struct State {
    text_value: String,
    text_id: widget::text_input::Id,
}

impl std::default::Default for State {
    fn default() -> State {
        State {
            text_value: "".to_string(),
            text_id: widget::text_input::Id::new("text_entry"),
        }
    }
}

impl State {
    fn update(&mut self, message: Message) -> Task<Message> {
        log::info("update fn run");
        match message {
            Message::TextInputChanged(content) => {
                self.text_value = content;
                Task::none()
            }
            Message::FocusTextInput => widget::text_input::focus(self.text_id.clone()),
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        // the heck is a '_
        log::info("view fn run");
        let text_input = widget::text_input("placeholder", &self.text_value)
            .id(self.text_id.clone())
            .on_input(Message::TextInputChanged);

        let root_continer = widget::container(text_input)
            .padding(10)
            .center(iced::Fill);


        root_continer.into()
    }
}

pub fn main() -> iced::Result {
    log::warn("aaa");
    iced::application("Lumin", State::update, State::view)
        .subscription(capture_keyboard_input_subscription)
        .level(iced::window::Level::AlwaysOnTop)
        .resizable(false)
        .decorations(false)
        .window_size((800.0, 1.0))
        .theme(|_s| iced::Theme::CatppuccinMocha)
        .run()
}

fn capture_keyboard_input_subscription(_state: &State) -> iced::Subscription<Message> {
    iced::window::open_events().map(|_id| Message::FocusTextInput)
}
