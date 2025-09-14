use iced::widget;

#[allow(dead_code)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
}

pub fn heading<'a>(level: HeadingLevel, text: String) -> widget::Text<'a> {
    let font_mult = match level {
        HeadingLevel::H1 => 3.0,
        HeadingLevel::H2 => 2.0,
        HeadingLevel::H3 => 1.5,
    };

    widget::text(text).size(iced::Settings::default().default_text_size * font_mult)
}
