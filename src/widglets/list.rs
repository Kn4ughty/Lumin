use iced::widget;

use super::PADDING;
use super::{HeadingLevel, heading};

const ICON_SIZE: f32 = 32.0;

// pub struct ResultList<Message> {
//     selected_index: usize,
//     rows: Vec<ListRow<Message>>,
// }

pub struct ListRow<Message> {
    text: String,
    subtext: Option<String>,
    icon: Option<iced::widget::image::Handle>,
    on_activate: Option<Message>,
}

impl<Message> ListRow<Message> {
    pub fn new<T>(text: T) -> Self
    where
        T: ToString,
    {
        Self {
            text: text.to_string(),
            subtext: None,
            icon: None,
            on_activate: None,
        }
    }

    pub fn subtext<T>(mut self, subtext: T) -> Self
    where
        T: ToString,
    {
        self.subtext = Some(subtext.to_string());
        self
    }

    pub fn optional_subtext<T>(self, subtext: Option<T>) -> Self
    where
        T: ToString,
    {
        if let Some(text) = subtext {
            self.subtext(text)
        } else {
            self
        }
    }

    pub fn icon(mut self, handle: widget::image::Handle) -> Self {
        self.icon = Some(handle);
        self
    }

    pub fn optional_icon(self, handle: Option<widget::image::Handle>) -> Self {
        if let Some(h) = handle {
            self.icon(h)
        } else {
            self
        }
    }

    /// Message to output when selected and enter is pressed, or clicked.
    pub fn on_activate(mut self, msg: Message) -> Self {
        self.on_activate = Some(msg);
        self
    }
}

impl<'a, Message> From<ListRow<Message>> for iced::Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: ListRow<Message>) -> Self {
        let mut row_widget = widget::Row::new().padding(0);

        let icon_widget = widget::Responsive::new(move |size| {
            // wish i didnt have to clone
            if let Some(icon) = value.icon.clone() {
                let real_image =
                    widget::image::Image::new(icon).content_fit(iced::ContentFit::Cover);

                widget::container(real_image)
            } else {
                widget::container(widget::image(super::MISSING_IMAGE.clone()))
            }
            .clip(true)
            .width(size.height)
            .height(size.height)
            .padding(0)
            .align_y(iced::Alignment::Center)
            .align_x(iced::Alignment::Center)
            .into()
        })
        .width(iced::Shrink)
        .height(iced::Length::Fixed(ICON_SIZE));

        row_widget = row_widget.push(icon_widget);
        row_widget = row_widget.push(widget::space().width(iced::Length::Fixed(PADDING)));

        let mut text_area = widget::column(vec![]);

        let main_name = widget::container(
            heading(HeadingLevel::H3, value.text, None)
                .align_x(iced::Left)
                .align_y(iced::Alignment::Center)
                .width(iced::Fill),
        );
        text_area = text_area.push(main_name);

        let subtext_widget = widget::container(
            heading(
                HeadingLevel::Subheading,
                value.subtext.unwrap_or_default(),
                None,
            )
            .align_x(iced::Right)
            .align_y(iced::Alignment::Center),
        );
        text_area = text_area.push(subtext_widget);

        row_widget = row_widget.push(text_area);

        widget::container(
            widget::button(row_widget)
                .width(iced::Fill)
                .height(iced::Shrink)
                .on_press_maybe(value.on_activate)
                .style(|theme, status| {
                    let mut button_style = widget::button::secondary(theme, status);

                    let ext_pallet = theme.extended_palette();
                    button_style.text_color = ext_pallet.background.base.text;
                    match status {
                        widget::button::Status::Hovered => {
                            button_style =
                                button_style.with_background(ext_pallet.secondary.weak.color);
                        }
                        widget::button::Status::Active | widget::button::Status::Pressed => {
                            button_style = button_style.with_background(iced::color!(0, 0, 0, 0.0));
                        }
                        _ => {}
                    }
                    button_style
                }),
        )
        .padding(PADDING)
        .into()
    }
}
