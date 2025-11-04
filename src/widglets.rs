use std::{fs, path::PathBuf};

use iced::widget;

use crate::module::ModuleMessage;

const PADDING: f32 = 4.0;
const SVG_HEIGHT: usize = 64;
const SVG_WIDTH: usize = 64;

pub async fn svg_path_to_handle(path: PathBuf) -> Result<iced::widget::image::Handle, String> {
    let contents = fs::read_to_string(path).map_err(|_e| "couldnt read path to string")?;
    let tree = resvg::usvg::Tree::from_str(&contents, &resvg::usvg::Options::default())
        .map_err(|_e| "Could not turn contents to tree")?;

    let mut data: Box<[u8]> = vec![0; SVG_HEIGHT * SVG_WIDTH * 4].into_boxed_slice();

    let mut pixmap = resvg::tiny_skia::PixmapMut::from_bytes(
        &mut data,
        SVG_WIDTH.try_into().expect("Cant fail"),
        SVG_HEIGHT.try_into().expect("Cant fail"),
    )
    .ok_or("Can create pixmap")?;

    let svg_size = tree.size();
    let transform = resvg::tiny_skia::Transform::from_scale(
        SVG_WIDTH as f32 / svg_size.width(),
        SVG_HEIGHT as f32 / svg_size.height(),
    );
    resvg::render(&tree, transform, &mut pixmap);

    Ok(iced::widget::image::Handle::from_rgba(
        SVG_WIDTH.try_into().expect("cant fail"),
        SVG_HEIGHT.try_into().expect("cant fail"),
        data,
    ))
}

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
    let mut row_widget = widget::Row::new().padding(0);

    let full_icon = widget::Responsive::new(move |size| {
        // wish i didnt have to clone
        if let Some(icon) = icon.clone() {
            let real_image = widget::image::Image::new(icon).content_fit(iced::ContentFit::Cover);

            widget::container(real_image)
        } else {
            // TODO. Get image from user icon theme
            widget::container(widget::image(widget::image::Handle::from_bytes(
                include_bytes!("../assets/image-missing-symbolic.png").to_vec(),
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

    row_widget = row_widget.push(full_icon);
    row_widget = row_widget.push(widget::space().width(iced::Length::Fixed(PADDING)));

    // let colw = widget::Column::new();

    let text_widget = widget::container(
        heading(HeadingLevel::H3, text, None)
            .align_x(iced::Left)
            .align_y(iced::Alignment::Center)
            .width(iced::Fill),
    );
    row_widget = row_widget.push(text_widget);
    // let colw = colw.push(text_widget);

    let subtext_widget = widget::container(
        heading(HeadingLevel::Subheading, subtext.unwrap_or("".into()), None)
            .align_x(iced::Right)
            .align_y(iced::Alignment::Center),
    );
    row_widget = row_widget.push(subtext_widget);
    // let colw = colw.push(subtext_widget);
    // row_widget = row_widget.push(colw);

    widget::container(
        widget::button(row_widget)
            .width(iced::Fill)
            .height(iced::Shrink)
            .on_press_maybe(on_press)
            .style(widget::button::secondary),
    )
    .padding(PADDING)
}
