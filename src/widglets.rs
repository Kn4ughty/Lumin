use crate::module::ModuleMessage;
use iced::widget;

const PADDING: f32 = 4.0;

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
        HeadingLevel::H3 => 1.25,
        HeadingLevel::Subheading => 1.0,
    };

    // let font_color = match level {
    //     HeadingLevel::Subheading => iced
    //     _ => None
    // }

    font.weight = match level {
        HeadingLevel::H1 | HeadingLevel::H2 => iced::font::Weight::Bold,
        _ => iced::font::Weight::Normal,
    };

    widget::text(text)
        .size(iced::Settings::default().default_text_size * font_mult)
        .style(move |theme: &iced::Theme| {
            let c = match level {
                HeadingLevel::Subheading => Some(theme.extended_palette().primary.weak.text),
                _ => None,
            };
            widget::text::Style { color: c }
        })
}

pub fn listrow<'a>(
    text: String,
    subtext: Option<String>,
    on_press: Option<ModuleMessage>,
    icon: Option<iced::widget::image::Handle>,
) -> widget::Container<'a, ModuleMessage> {
    let mut row_widget = widget::Row::new();

    if let Some(icon) = icon {
        let real_image = widget::image::Image::new(icon).content_fit(iced::ContentFit::Cover);
        row_widget = row_widget.push(
            widget::container(real_image)
                .style(widget::container::bordered_box)
                .height(iced::Shrink)
                .width(iced::Shrink)
                .clip(true),
        );
    };

    let text_widget = widget::container(
        heading(HeadingLevel::H3, text, None)
            .align_x(iced::Left)
            .width(iced::Fill),
    );
    row_widget = row_widget.push(text_widget);

    let subtext_widget = widget::container(
        heading(HeadingLevel::Subheading, subtext.unwrap_or("".into()), None).align_x(iced::Right),
    );
    row_widget = row_widget.push(subtext_widget);

    // log::debug!("row_widget was: {row_widget:?}");

    widget::container(
        widget::button(row_widget)
            .width(iced::Fill)
            .height(iced::Shrink)
            .on_press_maybe(on_press)
            .style(widget::button::secondary),
    )
    .padding(PADDING)
}
