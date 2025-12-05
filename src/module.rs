use iced::{Element, Task};
use serde::Deserialize;

use crate::message::Message;

use crate::apps;
use crate::calculator;
use crate::files;
use crate::websearch;

#[derive(Debug, Clone)]
pub enum ModuleMessage {
    TextChanged(String),
    SelectionUp,
    SelectionDown,
    ActivatedIndex(usize),
    AppMessage(apps::AppMessage),
    WebMessage(websearch::WebMsg),
    FileMessage(files::FileMsg),
    DoNothing,
}

pub trait Module {
    fn update(&mut self, msg: ModuleMessage) -> Task<ModuleMessage>;
    fn view(&self) -> Element<'_, ModuleMessage>;
    /// Executed when user presses the enter key
    fn run(&self) -> Task<Message>;
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
pub enum ModuleEnum {
    AppSearch,
    WebSearch,
    FileSearch,
    HelpScreen,
    Calculator,
}
impl ModuleEnum {
    pub fn description(&self) -> String {
        match self {
            Self::AppSearch => "Launch your installed apps",
            Self::WebSearch => "Quick access to search different websites",
            Self::FileSearch => "file search",
            Self::HelpScreen => "help",
            Self::Calculator => "calc",
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
            ModuleEnum::FileSearch => || Box::new(files::FileSearcher::new()),
            ModuleEnum::HelpScreen => panic!("not possible"),
        }
    }
}
