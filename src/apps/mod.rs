use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::Mutex;

use iced::Task;
use iced::widget;

pub mod desktop_entry;
pub mod mac_apps;

#[cfg(target_os = "linux")]
use desktop_entry as app_searcher;
#[cfg(target_os = "macos")]
use mac_apps as app_searcher;

use crate::constants;
use crate::module::{Module, ModuleMessage};
use crate::serworse;
use crate::util;
use crate::widglets;

// const ICON_LOOKUP_BATCH_AMOUNT: i32 = 4;

const APP_FREQUENCY_LOOKUP_RELPATH: &str = "app_lookup";
const ICON_CACHE_RELPATH: &str = "icon_cache";

// erhg
static ICON_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, PartialEq, Debug)]
pub struct App {
    cmd: String,
    icon: Option<Icon>,
    args: Vec<String>,
    working_dir: Option<String>,
    name: String,
    subname: Option<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Icon {
    ImageHandle(widget::image::Handle),
    NotFoundYet(String),
}

#[derive(Clone, Debug)]
pub enum AppMessage {
    IconLoaded(String, Option<(String, iced::widget::image::Handle)>),
}

pub struct AppModule {
    app_list: Vec<App>,
    app_frequencies: HashMap<String, u32>,
}

impl Default for AppModule {
    fn default() -> Self {
        Self::new()
    }
}

impl AppModule {
    // Duplicated logic betweeen new and open_app which is sad. should fix this
    pub fn new() -> Self {
        // attempt to load hashmap from disk
        let path_string = constants::DATA_DIR
            .get()
            .expect("DATA_DIR initialised")
            .to_owned()
            + APP_FREQUENCY_LOOKUP_RELPATH;
        let path = std::path::Path::new(&path_string);

        let mut freq_map: HashMap<String, u32> = HashMap::new();
        if let Ok(data) = std::fs::read_to_string(path) {
            match serworse::parse_csv::<u32>(&data) {
                Ok(map1) => freq_map = map1,
                Err(e) => log::error!("Could not read app_frequencies to hashmap. E: {e:#?}"),
            }
        } else {
            log::warn!(
                "Could not read app_frequencies to string.\
                Once any app is launched for the first time, \
                this warning should go away as the hashmap should have been written"
            );
        };

        let icon_cache_path = constants::CACHE_DIR
            .get()
            .expect("CACHE_DIR init'ed")
            .to_owned()
            + ICON_CACHE_RELPATH;

        if let Ok(data) = std::fs::read_to_string(icon_cache_path) {
            match serworse::parse_csv::<String>(&data) {
                Ok(disk_cache) => {
                    let mut main_map = ICON_CACHE.lock().expect("can get ICON_CACHE");
                    for (key, val) in disk_cache.iter() {
                        main_map.insert(key.to_string(), val.to_string());
                    }
                }
                Err(e) => log::error!("Could not read icon_cache to hashmap. E: {e:#?}"),
            }
        } else {
            log::warn!(
                "Could not read app_frequencies to string.\
                Once any app is launched for the first time, \
                this warning should go away as the hashmap should have been written"
            );
        };

        // let mut icon_map: HashMap<String, widget::image::Handle> = HashMap::new();

        AppModule {
            app_list: Vec::new(),
            app_frequencies: freq_map,
        }
    }

    fn run_app_at_index(&self, index: usize) {
        let Some(first) = self.app_list.get(index) else {
            return;
        };

        // Increment app frequency hashmap
        let mut map = self.app_frequencies.clone();
        if let Some(old_val) = map.get(&first.name) {
            map.insert(first.name.clone(), *old_val + 1);
        } else {
            map.insert(first.name.clone(), 1);
        }

        log::debug!("New app_frequencies hashmap is {map:#?}");

        let path_string = constants::DATA_DIR
            .get()
            .expect("DATA_DIR initialised")
            .to_owned()
            + APP_FREQUENCY_LOOKUP_RELPATH;
        let app_freq_path = std::path::Path::new(&path_string);

        if let Err(e) = std::fs::write(app_freq_path, serworse::hash_map_to_csv(map)) {
            log::error!(
                "Could not write new app frequency hashmap to file!! e: {e}\nHashmap is: {e:#?}"
            );
        } else {
            log::trace!("Successfully wrote to path: {app_freq_path:?}");
        };

        // Write icon_cache to disk

        let icon_cache_path = constants::CACHE_DIR
            .get()
            .expect("CACHE_DIR init'ed")
            .to_owned()
            + ICON_CACHE_RELPATH;

        let cache_map = ICON_CACHE.lock().expect("not poisoned").clone();

        if let Err(e) = std::fs::write(icon_cache_path, serworse::hash_map_to_csv(cache_map)) {
            log::error!("Could not write icon_cache to file!! e: {e}\nHashmap is: {e:#?}");
        } else {
            log::debug!("Successfully wrote to path: {app_freq_path:?}");
        };

        util::execute_command_detached(
            first.cmd.clone(),
            first.args.clone(),
            first.working_dir.clone(),
        )
        .expect("Can execute_command_detached");
    }
}

impl Module for AppModule {
    fn view(&self) -> iced::Element<'_, ModuleMessage> {
        widget::scrollable(
            widget::column(
                self.app_list
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(|(i, app)| {
                        let icon = match app.icon {
                            None => None,
                            Some(Icon::NotFoundYet(_)) => None,
                            Some(Icon::ImageHandle(h)) => Some(h),
                        };

                        widglets::listrow(
                            app.name,
                            app.subname,
                            Some(ModuleMessage::ActivatedIndex(i)),
                            icon,
                        )
                        .into()
                    }),
            )
            .width(iced::Fill),
        )
        .into()
    }

    fn update(&mut self, msg: ModuleMessage) -> Task<ModuleMessage> {
        match msg {
            ModuleMessage::TextChanged(input) => {
                if self.app_list.is_empty() {
                    log::trace!("Generating app_list");
                    let start = std::time::Instant::now();
                    self.app_list = app_searcher::get_apps();
                    log::info!(
                        "Time to get #{} apps: {:#?}",
                        self.app_list.len(),
                        start.elapsed()
                    )
                }

                let start = std::time::Instant::now();
                // Cached_key seems to be much faster which is interesting since text_value is
                // always changing
                let input = &input.to_lowercase();
                self.app_list.sort_by_cached_key(|app| {
                    let mut score = util::longest_common_substr(&app.name.to_lowercase(), input);
                    if app.name.to_lowercase().starts_with(input) {
                        score += 2;
                    }
                    if let Some(raw_freq) = self.app_frequencies.get(&app.name) {
                        score += (*raw_freq as f32).ln().max(0.0).floor() as i32;
                    }

                    -score
                });

                log::debug!(
                    "Time to sort #{} apps: {:#?}",
                    self.app_list.len(),
                    start.elapsed()
                );

                let start = std::time::Instant::now();

                let icons_to_lookup: Vec<&str> = self
                    .app_list
                    .iter()
                    .filter_map(|app| match &app.icon {
                        Some(Icon::NotFoundYet(a)) => Some(a.as_str()),
                        Some(Icon::ImageHandle(_)) => None,
                        None => None,
                    })
                    .collect();

                let tasks = icons_to_lookup.iter().map(|key| {
                    let k: String = key.to_string();
                    Task::perform(get_icon(k.clone()), move |handle| {
                        let k = k.clone();
                        ModuleMessage::AppMessage(AppMessage::IconLoaded(k, handle))
                    })
                });

                Task::batch(tasks).chain(Task::perform(std::future::ready(()), move |_| {
                    log::info!("Total time to get icons: {:#?}", start.elapsed());
                    ModuleMessage::DoNothing
                }))
            }
            ModuleMessage::ActivatedIndex(i) => {
                Self::run_app_at_index(self, i);
                iced::exit()
            }
            ModuleMessage::AppMessage(AppMessage::IconLoaded(key, res)) => {
                log::trace!("iconloaded: {key}");
                let start = iced::debug::time("IconLoaded");
                let icon_handle = if let Some((path, handle)) = res {
                    ICON_CACHE
                        .lock()
                        .expect("Can lock cache")
                        .insert(key.clone(), path.clone());

                    Some(Icon::ImageHandle(handle.clone()))
                } else {
                    // Failed to lookup icon for app
                    log::warn!("Failed to lookup icon: key: {key}");
                    None
                };

                self.app_list
                    .iter_mut()
                    .filter_map(|app| match &app.icon.clone() {
                        Some(Icon::NotFoundYet(key)) => Some((key.clone(), app)),
                        Some(Icon::ImageHandle(_)) => None,
                        None => None,
                    })
                    .for_each(|(app_key, app)| {
                        // log::debug!("Comparing {app_key} with {app:?}");
                        if key == *app_key {
                            log::trace!("Updating app: {app:?}");
                            app.icon = icon_handle.clone()
                        }
                    });

                start.finish();

                Task::none()
            }
            x => {
                log::trace!("App module received irrelevant msg: {x:?}");
                Task::none()
            }
        }
    }

    fn run(&self) -> Task<crate::message::Message> {
        Self::run_app_at_index(self, 0);
        iced::exit()
    }
}

async fn get_icon(icon_name: String) -> Option<(String, iced::widget::image::Handle)> {
    let start = iced::debug::time("GetIconTime");

    let mut final_path = PathBuf::new();

    if let Some(icon_path) = ICON_CACHE
        .lock()
        .expect("Can unlock")
        .get(&icon_name)
        .cloned()
    {
        log::trace!("Cache hit! name: {icon_name}");

        final_path = icon_path.clone().into();
    } else {
        log::trace!("Cache miss! name: {icon_name}");

        let copy = icon_name.clone();
        let icon_path = tokio::task::spawn_blocking(move || app_searcher::load_icon(copy))
            .await
            .ok()
            .flatten();

        if let Some(path) = &icon_path {
            let path_str = path
                .to_str()
                .expect("icon_path is valid unicode")
                .to_owned();

            ICON_CACHE
                .lock()
                .expect("Can unlock")
                .insert(icon_name.to_owned(), path_str.clone());
            final_path = path.to_path_buf();
        }
    }

    let final_image_handle = {
        if final_path.extension() == Some(std::ffi::OsStr::new("svg")) {
            match widglets::svg_path_to_handle(final_path.clone()).await {
                Ok(image) => Some(image),
                Err(e) => {
                    log::warn!("Failed to load SVG at {final_path:?} with error: {e}");
                    None
                }
            }
        } else {
            Some(iced::widget::image::Handle::from_path(final_path.clone()))
        }
    }?;

    start.finish();

    if final_path == PathBuf::new() {
        None
    } else {
        Some((final_path.to_str()?.to_string(), final_image_handle))
    }
}
