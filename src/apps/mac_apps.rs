use icns;
use log;
use std::io::BufReader;
use std::path::Path;
use std::{fs::File, path::PathBuf};
use walkdir::{DirEntry, WalkDir};

use iced::advanced::image::Handle;

use super::{App, Icon, OSAppSearcher};

#[derive(Default)]
pub struct MacOsAppSearcher {}

impl OSAppSearcher for MacOsAppSearcher {
    fn get_apps(&self) -> Vec<App> {
        let m_apps = load_all_apps();
        m_apps
            .iter()
            .map(|a| App {
                name: a.name.clone(),
                icon: Some(Icon::NotFoundYet(a.path.clone())),
                cmd: "open".into(),
                args: vec![a.path.clone()],
                working_dir: None,
                subname: None,
            })
            .collect()
    }

    fn load_icon_path(&self, s: String) -> Option<PathBuf> {
        // TODO. Don't hardcode path, and read info.plist file
        let icon_path = format!("{s}/Contents/Resources/AppIcon.icns");
        Some(icon_path.into())
    }

    fn load_icon_image(&self, path: &Path) -> Option<iced::widget::image::Handle> {
        log::trace!("generated path for the appIconicns is {path:?}");

        let file = BufReader::new(
            File::open(path)
                .map_err(|_| MacAppError::IconPathWasWrong)
                .ok()?,
        );

        let icon_family = icns::IconFamily::read(file).expect("Can read iconfile");
        let avail_icons = icon_family.available_icons();
        let icns_image = icon_family
            .get_icon_with_type(*avail_icons.first().expect("App should have an icon"))
            .expect("Can get_icon_with_type. Data May be malformed");
        log::trace!("mac icon type is {:?}", icns_image.pixel_format());
        let iced_image: iced::advanced::image::Handle = Handle::from_rgba(
            icns_image.width(),
            icns_image.height(),
            icns_image.into_data(),
        );
        Some(iced_image)
    }
}

pub struct MacApp {
    pub name: String,
    pub path: String,
}

enum MacAppError {
    IconPathWasWrong,
}

impl MacApp {
    fn new_from_path(path: String) -> Result<MacApp, MacAppError> {
        log::trace!("making new app from path: {path}");

        let name = path
            .split('/')
            .next_back()
            .expect("Should be able to split up path and has last")
            .to_string();

        // Remove the .app from filename
        let name = name[0..(name.len() - 4)].to_string();

        Ok(Self { name, path })
    }
}

fn is_app(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".app"))
        .unwrap_or(false)
}

pub fn load_all_apps() -> Vec<MacApp> {
    let mut full_apps = Vec::new();
    // There is probably a better way to do it that isnt hardcoding.
    for dir in ["/Applications/", "/System/Applications/"] {
        WalkDir::new(dir)
            .max_depth(4)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
            .filter(is_app)
            .map(|e| {
                MacApp::new_from_path(e.path().to_str().expect("Path is valid utf8").to_string())
            })
            .filter_map(|e| e.ok())
            .for_each(|e| full_apps.push(e))
    }
    full_apps
}
