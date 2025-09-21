use crate::module::ModuleMessage;
use iced::widget;

#[allow(dead_code)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    Subheading,
}

pub fn heading<'a>(
    level: HeadingLevel,
    text: String,
    font: Option<iced::Font>,
) -> widget::Text<'a> {
    let mut font = font.unwrap_or(iced::Font::DEFAULT);

    let font_mult = match level {
        HeadingLevel::H1 => 3.0,
        HeadingLevel::H2 => 2.0,
        HeadingLevel::H3 => 1.5,
        HeadingLevel::Subheading => 1.0,
    };

    font.weight = match level {
        HeadingLevel::H1 | HeadingLevel::H2 => iced::font::Weight::Bold,
        _ => iced::font::Weight::Normal,
    };

    widget::text(text).size(iced::Settings::default().default_text_size * font_mult)
}

pub fn listrow<'a>(
    text: String,
    subtext: Option<String>,
    _icon: Option<String>,
) -> widget::Container<'a, ModuleMessage> {
    let text_widget = heading(HeadingLevel::H2, text, None);
    let subtext_widget = heading(HeadingLevel::Subheading, subtext.unwrap_or("".into()), None);

    // widget::container::Container
    widget::container(widget::row![text_widget, subtext_widget])
        .style(widget::container::rounded_box)
        .width(iced::Fill)
}

pub fn listbox<'a, I>(t: I) -> iced::Element<'a, ModuleMessage>
where
    I: IntoIterator,
    I::Item: Into<iced::Element<'a, ModuleMessage>>,
{
    let elements: Vec<_> = t.into_iter().map(Into::into).collect();
    widget::scrollable(widget::column(elements))
        .width(iced::Fill)
        .into()
}
