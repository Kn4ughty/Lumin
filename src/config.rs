use serde::Deserialize;
use std::{collections::HashMap, sync::LazyLock};

use iced::Theme;

use crate::constants;

static CONFIG_PATH: LazyLock<String> =
    LazyLock::new(|| constants::CONFIG_DIR.clone() + "config.toml");

static DEFAULT_SETTINGS: LazyLock<Settings> =
    LazyLock::new(
        || match toml::from_str(include_str!("../assets/config.toml")) {
            Err(e) => panic!(
                "{}",
                format!("Can turn default config into Settings. Error: {e:#?}")
            ),
            Ok(o) => o,
        },
    );

#[test]
fn default_settings_work() {
    // Just so it does something
    println!("{:#?}", DEFAULT_SETTINGS.color_scheme);
}

pub static SETTINGS: LazyLock<Settings> = LazyLock::new(|| {
    load_from_disk().unwrap_or_else(|e| {
        log::error!(
            "User config was invalid!! {e:#?}
===========
You can attempt to copy the default config manually so it has the new fields.
it is in `assets/config.toml`"
        );
        LazyLock::<Settings>::force(&DEFAULT_SETTINGS).clone()
    })
});

#[derive(Debug)]
enum ConfigError {
    CannotReadConfig,
    #[allow(dead_code, reason = "Used for debug output in terminal")]
    TomlError(toml::de::Error),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    #[serde(with = "ThemeDef")]
    pub color_scheme: iced::Theme,
    pub transparent_background: bool,
    pub file_settings: FileSettings,
    pub app_prefixes: HashMap<crate::module::ModuleEnum, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileSettings {
    pub search_directories: Vec<String>,
}

fn load_from_disk() -> Result<Settings, ConfigError> {
    if !std::fs::exists(CONFIG_PATH.clone()).is_ok_and(|v| v) {
        match std::fs::write(CONFIG_PATH.clone(), include_str!("../assets/config.toml")) {
            Ok(_) => log::info!(
                "Successfuly wrote default config to file system to location: {CONFIG_PATH:?}"
            ),
            Err(e) => log::error!("Could not write default config to fs! {e:?}"),
        };
    }

    let raw_string =
        std::fs::read_to_string(CONFIG_PATH.clone()).map_err(|_| ConfigError::CannotReadConfig)?;

    #[allow(
        clippy::redundant_closure,
        reason = "False positive. Breaks strangely if fixed"
    )]
    let config: Settings = toml::from_str(&raw_string).map_err(|e| ConfigError::TomlError(e))?;

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
