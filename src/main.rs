use iced::{Task, widget};

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
        println!("update fn run");
        match message {
            Message::TextInputChanged(content) => {
                self.text_value = content;
                Task::none()
            }
            Message::FocusTextInput => {
                widget::text_input::focus(self.text_id.clone())
            }
        }
    }

    fn view(&self) -> widget::Column<'_, Message> {
        // the heck is a '_
        println!("view fn run");
        let text_input = widget::text_input("placeholder", &self.text_value)
            .on_input(Message::TextInputChanged)
            .id(self.text_id.clone())
            .on_input(Message::TextInputChanged);

        widget::column![text_input,]
    }
}

pub fn main() -> iced::Result {
    iced::application("Lumin", State::update, State::view)
        .subscription(capture_keyboard_input_subscription)
        .level(iced::window::Level::AlwaysOnTop)
        .resizable(false)
        .decorations(false)
        .run()
}

fn capture_keyboard_input_subscription(_state: &State) -> iced::Subscription<Message> {
    iced::window::open_events().map(|_id| Message::FocusTextInput)
}
