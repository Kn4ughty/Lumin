use icns;
use log;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use iced::advanced::image::Handle;

pub struct MacApp {
    pub name: String,
    pub path: String,
    pub icon_image: Option<iced::widget::Image>,
}

impl MacApp {
    fn new_from_path(path: String) -> MacApp {
        // app_root/Contents/AppIcon.icns
        // TODO. Dont just unwrap things. Actually return an error.
        // Also, cannot hardcode icon file name. need to get it from
        // Info.plist
        // log::trace!("making new app from path: {path}");
        // let icon_path = format!("{path}/Contents/Resources/AppIcon.icns");
        // log::trace!("generated path for the appIconicns is {icon_path}");
        //
        // let file = BufReader::new(File::open(icon_path).unwrap());
        // let icon_family = icns::IconFamily::read(file).unwrap();
        // let avail_icons = icon_family.available_icons();
        // let icns_image = icon_family
        //     .get_icon_with_type(*avail_icons.get(0).expect("App should have an icon"))
        //     .unwrap();
        // // Hope this works!
        // let iced_image: iced::widget::Image<Handle> =
        //     iced::widget::Image::new(Handle::from_bytes(icns_image.into_data()));
        let name = path
            .split('/')
            .last()
            .expect("Should be able to split up path and has last")
            .to_string();

        Self {
            name,
            path,
            icon_image: None,
        }
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
        for thingy in WalkDir::new(dir)
            .max_depth(4)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
            .filter(|e| is_app(e))
            .map(|e| MacApp::new_from_path(e.path().to_str().unwrap().to_string()))
        {
            full_apps.push(thingy)
        }
    }
    full_apps
}
