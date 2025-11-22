use serde::Deserialize;
use std::sync::LazyLock;

use iced::Theme;

use crate::constants;

static CONFIG_PATH: LazyLock<String> = LazyLock::new(|| constants::CONFIG_DIR.clone() + "conf.csv");

const DEFAULT_CONFIG: LazyLock<Settings> = LazyLock::new(|| {
    toml::toml! {
        color_scheme = "CatppuccinMocha"
        transparent_background = false
    }
    .try_into()
    .expect("Can turn default config into Settings")
});

pub static SETTINGS: LazyLock<Settings> =
    LazyLock::new(|| load_from_disk().expect("Can load settings!"));

#[derive(Debug)]
enum ConfigError {
    FileSystemError,
}

#[derive(Clone, Deserialize)]
pub struct Settings {
    #[serde(with = "ThemeDef")]
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

// impl Settings {
//     fn optional_theme(mut self, raw_theme: Option<&String>) -> Self {
//         let Some(raw_theme) = raw_theme else {
//             return self;
//         };
//
//         let new = match raw_theme.as_str() {
//             "CatppuccinMocha" => Theme::CatppuccinMocha,
//             "CatppuccinLatte" => Theme::CatppuccinLatte,
//             _ => self.color_scheme, // default value
//         };
//         self.color_scheme = new;
//
//         self
//     }
// }

fn load_from_disk() -> Result<Settings, ConfigError> {
    let raw_string =
        std::fs::read_to_string(CONFIG_PATH.clone()).map_err(|_| ConfigError::FileSystemError)?;

    let config: Settings = match toml::from_str(&raw_string) {
        Ok(t) => t,
        Err(e) => {
            log::error!("User config was invalid!! {e}");
            let conf = DEFAULT_CONFIG;
            LazyLock::<Settings>::force(&conf).clone()
        }
    };

    Ok(config)
}

// This is nasty. However, it is less bad than doing it manually.
// https://serde.rs/remote-derive.html
// This is copy pasted from the iced source code
#[derive(Clone, Deserialize)]
#[serde(remote = "Theme")]
enum ThemeDef {
    /// The built-in light variant.
    Light,
    /// The built-in dark variant.
    Dark,
    /// The built-in Dracula variant.
    Dracula,
    /// The built-in Nord variant.
    Nord,
    /// The built-in Solarized Light variant.
    SolarizedLight,
    /// The built-in Solarized Dark variant.
    SolarizedDark,
    /// The built-in Gruvbox Light variant.
    GruvboxLight,
    /// The built-in Gruvbox Dark variant.
    GruvboxDark,
    /// The built-in Catppuccin Latte variant.
    CatppuccinLatte,
    /// The built-in Catppuccin Frappé variant.
    CatppuccinFrappe,
    /// The built-in Catppuccin Macchiato variant.
    CatppuccinMacchiato,
    /// The built-in Catppuccin Mocha variant.
    CatppuccinMocha,
    /// The built-in Tokyo Night variant.
    TokyoNight,
    /// The built-in Tokyo Night Storm variant.
    TokyoNightStorm,
    /// The built-in Tokyo Night Light variant.
    TokyoNightLight,
    /// The built-in Kanagawa Wave variant.
    KanagawaWave,
    /// The built-in Kanagawa Dragon variant.
    KanagawaDragon,
    /// The built-in Kanagawa Lotus variant.
    KanagawaLotus,
    /// The built-in Moonfly variant.
    Moonfly,
    /// The built-in Nightfly variant.
    Nightfly,
    /// The built-in Oxocarbon variant.
    Oxocarbon,
    /// The built-in Ferra variant:
    Ferra,
    // A [`Theme`] that uses a [`Custom`] palette.
    // I dont have this, will it still work?
    // Custom(Arc<Custom>),
}
