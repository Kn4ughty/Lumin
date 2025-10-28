use crate::module::ModuleMessage;

#[derive(Clone, Debug)]
pub enum Message {
    TextInputChanged(String),
    FocusTextInput,
    TextInputSubmitted(String),
    PluginMessage(ModuleMessage),
    Close,
}
