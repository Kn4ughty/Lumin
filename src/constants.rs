use std::sync::LazyLock;

static HOME_DIR: LazyLock<String> =
    LazyLock::new(|| std::env::var("HOME").expect("Can get home enviroment variable"));

pub static DATA_DIR: LazyLock<String> = LazyLock::new(|| {
    let path_str = HOME_DIR.clone() + "/.local/share/lumin/";

    if !std::fs::exists(&path_str).expect("Can check if data_path exists") {
        log::info!("Data dir not found. Creating at path {path_str:?}");
        std::fs::create_dir_all(&path_str).expect("Could create DATA_DIR");
    }
    path_str
});

pub static CACHE_DIR: LazyLock<String> = LazyLock::new(|| {
    let cache_dir = HOME_DIR.clone() + "/.cache/lumin/";

    if !std::fs::exists(&cache_dir).expect("Can check if cache_dir exists") {
        log::info!("cache_dir not found. Creating at path {cache_dir:?}");
        std::fs::create_dir_all(&cache_dir).expect("Could create cache_dir");
    }
    cache_dir
});

pub static CONFIG_DIR: LazyLock<String> = LazyLock::new(|| {
    let config_dir = HOME_DIR.clone() + "/.config/lumin/";

    if !std::fs::exists(&config_dir).expect("Can check if config_dir exists") {
        log::info!("Config dir not found. Creating at path {config_dir:?}");
        std::fs::create_dir_all(&config_dir).expect("Could create config_dir");
    }
    config_dir
});
