use std::{fs, path::PathBuf, sync::LazyLock};

use iced::widget;

pub mod list;
pub use list::ListRow;

const PADDING: f32 = 4.0;
const SVG_HEIGHT: usize = 64;
const SVG_WIDTH: usize = 64;

// TODO. Get image from user icon theme
static MISSING_IMAGE: LazyLock<iced::widget::image::Handle> = LazyLock::new(|| {
    widget::image::Handle::from_bytes(
        include_bytes!("../../assets/image-missing-symbolic.png").to_vec(),
    )
});

/// Uses resvg library to convert `path` to an image, with scaling to fit in bounds.
/// # Errors
/// Either, could not load from file, or there are errors in the SVG.
pub fn svg_path_to_handle(path: PathBuf) -> Result<iced::widget::image::Handle, String> {
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
/// Does appropriate sizing, weighting and colouring for a given heading level
pub fn heading<'a>(
    level: HeadingLevel,
    text: String,
    font: Option<iced::Font>,
) -> widget::Text<'a> {
    let mut font = font.unwrap_or(iced::Font::DEFAULT);

    let font_mult = match level {
        HeadingLevel::H1 => 2.0,
        HeadingLevel::H2 => 1.5,
        HeadingLevel::H3 => 1.0,
        HeadingLevel::Subheading => 0.7,
    };

    font.weight = match level {
        HeadingLevel::H1 | HeadingLevel::H2 => iced::font::Weight::Bold,
        _ => iced::font::Weight::Normal,
    };

    widget::text(text)
        .size(iced::Settings::default().default_text_size * font_mult)
        .style(move |_theme: &iced::Theme| {
            let c = match level {
                HeadingLevel::Subheading => Some(iced::color!(140, 140, 150)),
                _ => None,
            };
            widget::text::Style { color: c }
        })
}
