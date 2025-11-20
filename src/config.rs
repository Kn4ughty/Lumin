use std::sync::LazyLock;

use iced::Theme;

use crate::{constants, serworse};

static CONFIG_PATH: LazyLock<String> = LazyLock::new(|| constants::CONFIG_DIR.clone() + "conf.csv");

#[derive(Clone)]
pub struct Settings {
    pub color_scheme: iced::Theme,
    pub transparent_background: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            color_scheme: iced::Theme::CatppuccinMocha,
            transparent_background: false,
        }
    }
}

impl Settings {
    fn optional_theme(mut self, raw_theme: Option<&String>) -> Self {
        let Some(raw_theme) = raw_theme else {
            return self;
        };

        let new = match raw_theme.as_str() {
            "CatppuccinMocha" => Theme::CatppuccinMocha,
            "CatppuccinLatte" => Theme::CatppuccinLatte,
            _ => self.color_scheme, // default value
        };
        self.color_scheme = new;

        self
    }

    fn optional_transparent_background(mut self, new_raw: Option<&String>) -> Self {
        if let Some(new) = new_raw {
            self.transparent_background = new.to_lowercase() == "true"
        };
        self
    }
}

fn load_from_disk() -> Option<Settings> {
    if !std::fs::exists(CONFIG_PATH.clone()).expect("can check if config file exists") {
        return Some(Settings::default());
    }

    let raw_string = std::fs::read_to_string(CONFIG_PATH.clone()).expect("Can read config file");
    let disk_csv = serworse::parse_csv(raw_string.as_str()).ok()?;

    let settings_new = Settings::default()
        .optional_theme(disk_csv.get("theme"))
        .optional_transparent_background(disk_csv.get("transparent_background"));

    Some(settings_new)
}

pub static SETTINGS: LazyLock<Settings> =
    LazyLock::new(|| load_from_disk().expect("Can load settings!"));
