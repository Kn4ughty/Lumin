use crate::module::ModuleMessage;

#[derive(Clone, Debug)]
pub enum Message {
    TextInputChanged(String),
    ShouldDrag,
    WindowOpened(iced::window::Id),
    TextInputSubmitted(String),
    #[allow(clippy::enum_variant_names)]
    PluginMessage(ModuleMessage),
    Close,
    KeyboardUp,
    KeyboardDown,
    FontLoaded(Result<(), iced::font::Error>),
}
