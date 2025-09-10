use iced::Element;

pub trait Module {
    fn update(&mut self, input: &str);
    fn view(&self) -> Element<'_, String>; // why is string here?
    fn run(&self);
}
