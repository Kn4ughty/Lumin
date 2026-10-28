use iced::{Element, Task};
use serde::Deserialize;

use crate::message::Message;

use crate::apps;
use crate::calculator;
use crate::websearch;

#[derive(Debug, Clone)]
pub enum ModuleMessage {
    TextChanged(String),
    SelectionUp,
    SelectionDown,
    ActivatedIndex(usize),
    AppMessage(apps::AppMessage),
    WebMessage(websearch::WebMsg),
    DoNothing,
}

pub trait Module {
    fn update(&mut self, msg: ModuleMessage) -> Task<ModuleMessage>;
    fn view(&self) -> Element<'_, ModuleMessage>;
    /// Executed when user presses the enter key
    fn run(&self) -> Task<Message>;
}

#[derive(Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub enum ModuleEnum {
    AppSearch,
    WebSearch,
    HelpScreen,
    Calculator,
}
impl ModuleEnum {
    pub fn description(&self) -> String {
        match self {
            Self::AppSearch => "Launch your installed apps",
            Self::WebSearch => "Quick access to search different websites",
            Self::HelpScreen => "This help screen",
            Self::Calculator => "Basic calculator ",
        }
        .to_string()
    }
}

impl From<&ModuleEnum> for fn() -> Box<dyn Module> {
    fn from(value: &ModuleEnum) -> Self {
        match value {
            ModuleEnum::AppSearch => || Box::new(apps::AppModule::new()),
            ModuleEnum::WebSearch => || Box::new(websearch::Web::new()),
            ModuleEnum::Calculator => || Box::new(calculator::Calc::new()),
            ModuleEnum::HelpScreen => unreachable!("Help screen should not be ran as real module"),
        }
    }
}
