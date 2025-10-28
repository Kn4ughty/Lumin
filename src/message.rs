use crate::module::ModuleMessage;

#[derive(Clone, Debug)]
pub enum Message {
    TextInputChanged(String),
    ShouldDrag,
    WindowOpened(iced::window::Id),
    TextInputSubmitted(String),
    PluginMessage(ModuleMessage),
    Close,
}
