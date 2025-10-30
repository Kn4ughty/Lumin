#![allow(dead_code, reason = "Compile time importing shennanigans")]

use icns;
use log;
use std::fs::File;
use std::io::BufReader;
use walkdir::{DirEntry, WalkDir};

use iced::advanced::image::Handle;

use super::App;

pub fn get_apps() -> Vec<App> {
    let m_apps = load_all_apps();
    m_apps
        .iter()
        .map(|a| App {
            name: a.name.clone(),
            icon: a.icon_image.clone(),
            cmd: "open".into(),
            args: vec![a.path.clone()],
            working_dir: None,
            subname: None,
        })
        .collect()
}

pub struct MacApp {
    pub name: String,
    pub path: String,
    pub icon_image: Option<iced::advanced::image::Handle>,
}

enum MacAppError {
    IconPathWasWrong,
}

impl MacApp {
    fn new_from_path(path: String) -> Result<MacApp, MacAppError> {
        log::trace!("making new app from path: {path}");

        let icon_image = Self::get_app_icon(&path).ok();

        let name = path
            .split('/')
            .last()
            .expect("Should be able to split up path and has last")
            .to_string();

        // Remove the .app from filename
        let name = name[0..(name.len() - 4)].to_string();

        Ok(Self {
            name,
            path,
            icon_image,
        })
    }

    fn get_app_icon(path: &str) -> Result<Handle, MacAppError> {
        // TODO. Don't hardcode path, and read info.plist file
        let icon_path = format!("{path}/Contents/Resources/AppIcon.icns");
        log::trace!("generated path for the appIconicns is {icon_path}");

        let file =
            BufReader::new(File::open(icon_path).map_err(|_| MacAppError::IconPathWasWrong)?);
        let icon_family = icns::IconFamily::read(file).unwrap();
        let avail_icons = icon_family.available_icons();
        let icns_image = icon_family
            .get_icon_with_type(*avail_icons.get(0).expect("App should have an icon"))
            .unwrap();
        log::trace!("mac icon type is {:?}", icns_image.pixel_format());
        let iced_image: iced::advanced::image::Handle = Handle::from_rgba(
            icns_image.width(),
            icns_image.height(),
            icns_image.into_data(),
        );
        Ok(iced_image)
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
    for dir in vec!["/Applications/", "/System/Applications/"] {
        WalkDir::new(dir)
            .max_depth(4)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
            .filter(|e| is_app(e))
            .map(|e| MacApp::new_from_path(e.path().to_str().unwrap().to_string()))
            .filter_map(|e| e.ok())
            .for_each(|e| full_apps.push(e))
    }
    full_apps
}
