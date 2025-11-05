use iced::widget;

use super::PADDING;
use super::{HeadingLevel, heading};

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
                // TODO. Get image from user icon theme
                widget::container(widget::image(widget::image::Handle::from_bytes(
                    include_bytes!("../../assets/image-missing-symbolic.png").to_vec(),
                )))
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
        .height(iced::Length::Fixed(32.0)); // i dont like this

        row_widget = row_widget.push(icon_widget);
        row_widget = row_widget.push(widget::space().width(iced::Length::Fixed(PADDING)));

        // let colw = widget::Column::new();

        let text_widget = widget::container(
            heading(HeadingLevel::H3, value.text, None)
                .align_x(iced::Left)
                .align_y(iced::Alignment::Center)
                .width(iced::Fill),
        );
        row_widget = row_widget.push(text_widget);
        // let colw = colw.push(text_widget);

        let subtext_widget = widget::container(
            heading(
                HeadingLevel::Subheading,
                value.subtext.unwrap_or_default(),
                None,
            )
            .align_x(iced::Right)
            .align_y(iced::Alignment::Center),
        );
        row_widget = row_widget.push(subtext_widget);

        widget::container(
            widget::button(row_widget)
                .width(iced::Fill)
                .height(iced::Shrink)
                .on_press_maybe(value.on_activate)
                .style(widget::button::secondary),
        )
        .padding(PADDING)
        .into()
    }
}
