use std::collections::HashMap;
use std::ops::Mul;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::Mutex;

use iced::Task;
use iced::widget;

pub mod desktop_entry;
pub mod mac_apps;

use crate::constants;
use crate::module::{Module, ModuleMessage};
use crate::serworse;
use crate::sorting;
use crate::util;
use crate::widglets;

#[derive(Clone, PartialEq, Debug)]
pub struct App {
    cmd: String,
    args: Vec<String>,
    working_dir: Option<String>,
    name: String,
    subname: Option<String>,
    icon: Option<Icon>,
}

pub trait OSAppSearcher: Sync + Send {
    fn get_apps(&self) -> Vec<App>;
    fn load_icon_path(&self, s: String) -> Option<PathBuf>;
    fn load_icon_image(&self, path: &Path) -> Option<widget::image::Handle>;
}

#[derive(Clone, PartialEq, Debug)]
/// Icon Metatype to handle all cases
pub enum Icon {
    ImageHandle(widget::image::Handle),
    /// i.e not found yet, just path on disk
    NotFoundYet(String),
}

#[derive(Clone, Debug)]
pub enum AppMessage {
    /// Fetched an icon from disk. Path is given so it can be used in the `ICON_CACHE`
    IconLoaded(String, Option<(String, iced::widget::image::Handle)>),
}

pub struct AppModule {
    app_list: Vec<App>,
    /// How many times has the app been run before. Used for search score weighting.
    app_frequencies: HashMap<String, u32>,
    /// To prevent searching again if already searching
    have_started_icon_search: bool,
    /// Highlighted index to launch
    selected_index: usize,
}

static APP_SEARCHER: LazyLock<Box<dyn OSAppSearcher>> = LazyLock::new(|| {
    let searcher: Box<dyn OSAppSearcher> = if cfg!(target_os = "linux") {
        Box::new(desktop_entry::LinuxAppSearcher::default())
    } else if cfg!(target_os = "macos") {
        Box::new(mac_apps::MacOsAppSearcher::default())
    } else {
        panic!("Unknown operating system")
    };
    searcher
});

const APP_FREQUENCY_LOOKUP_RELPATH: &str = "app_lookup";
const ICON_CACHE_RELPATH: &str = "icon_cache";

static APP_FREQUENCY_FILE_PATH: LazyLock<String> =
    LazyLock::new(|| constants::DATA_DIR.to_owned() + APP_FREQUENCY_LOOKUP_RELPATH);

static ICON_CACHE_FILE_PATH: LazyLock<String> =
    LazyLock::new(|| constants::CACHE_DIR.to_owned() + ICON_CACHE_RELPATH);

// Big type name!
static ICON_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

impl Default for AppModule {
    fn default() -> Self {
        Self::new()
    }
}

impl AppModule {
    // Duplicated logic betweeen new and open_app which is sad. should fix this
    pub fn new() -> Self {
        // attempt to load hashmap from disk
        let mut freq_map: HashMap<String, u32> = HashMap::new();
        if let Ok(data) = std::fs::read_to_string(APP_FREQUENCY_FILE_PATH.clone()) {
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

        if let Ok(data) = std::fs::read_to_string(ICON_CACHE_FILE_PATH.clone()) {
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
                "Could not read Icon to string.\
                Once any app is launched for the first time, \
                this warning should go away as the hashmap should have been written"
            );
        };

        // let mut icon_map: HashMap<String, widget::image::Handle> = HashMap::new();

        AppModule {
            app_list: Vec::new(),
            app_frequencies: freq_map,
            have_started_icon_search: false,
            selected_index: 0,
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

        if let Err(e) = std::fs::write(
            APP_FREQUENCY_FILE_PATH.clone(),
            serworse::hash_map_to_csv(map),
        ) {
            log::error!(
                "Could not write new app frequency hashmap to file!! e: {e}\nHashmap is: {e:#?}"
            );
        } else {
            log::debug!("Successfully wrote to path: {APP_FREQUENCY_FILE_PATH:?}");
        };

        // Write icon_cache to disk

        let cache_map = ICON_CACHE.lock().expect("not poisoned").clone();

        if let Err(e) = std::fs::write(
            ICON_CACHE_FILE_PATH.clone(),
            serworse::hash_map_to_csv(cache_map),
        ) {
            log::error!("Could not write icon_cache to file!! e: {e}\nHashmap is: {e:#?}");
        } else {
            log::debug!("Successfully wrote to path: {ICON_CACHE_FILE_PATH:?}");
        };

        util::execute_command_detached(
            first.cmd.clone(),
            first.args.clone(),
            first.working_dir.clone(),
        )
        .expect("Can execute_command_detached");
    }

    fn handle_text_change(&mut self, input: String) -> Task<ModuleMessage> {
        self.selected_index = 0;

        if self.app_list.is_empty() {
            log::trace!("Generating app_list");
            let start = std::time::Instant::now();
            self.app_list = APP_SEARCHER.get_apps();
            log::info!(
                "Time to get #{} apps: {:#?}",
                self.app_list.len(),
                start.elapsed()
            )
        }

        let start = std::time::Instant::now();
        // Cached_key seems to be much faster which is interesting since text_value is
        // always changing
        // let input = &input.to_lowercase();
        self.app_list.sort_by_cached_key(|app| {
            let mut score = sorting::score_element(&input, &app.name);

            if let Some(raw_freq) = self.app_frequencies.get(&app.name) {
                // Preview: https://www.desmos.com/calculator/vyac5ua1as
                score += (*raw_freq as f32).ln().mul(0.5).max(0.0).floor() as i32;
            }

            -score
        });

        log::debug!(
            "Time to sort #{} apps: {:#?}",
            self.app_list.len(),
            start.elapsed()
        );

        self.do_icon_lookup()
    }

    fn do_icon_lookup(&mut self) -> Task<ModuleMessage> {
        let start = std::time::Instant::now();

        if self.have_started_icon_search {
            return Task::none();
        }
        self.have_started_icon_search = true;

        let icons_to_lookup: Vec<&str> = self
            .app_list
            .iter()
            .filter_map(|app| match &app.icon {
                Some(Icon::NotFoundYet(a)) => Some(a.as_str()),
                Some(Icon::ImageHandle(_)) => None,
                None => None,
            })
            .collect();

        if icons_to_lookup.is_empty() {
            return Task::none();
        }

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

    fn handle_icon_loaded(
        &mut self,
        key: String,
        res: Option<(String, widget::image::Handle)>,
    ) -> Task<ModuleMessage> {
        log::debug!("iconloaded: {key}");
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

                        widglets::ListRow::new(app.name)
                            .optional_subtext(app.subname)
                            .on_activate(ModuleMessage::ActivatedIndex(i))
                            .optional_icon(icon)
                            .selected(self.selected_index == i)
                            .into()
                    }),
            )
            .width(iced::Fill),
        )
        .direction(widget::scrollable::Direction::Vertical(
            widget::scrollable::Scrollbar::hidden(),
        ))
        .into()
    }

    fn update(&mut self, msg: ModuleMessage) -> Task<ModuleMessage> {
        match msg {
            ModuleMessage::TextChanged(input) => Self::handle_text_change(self, input),
            ModuleMessage::AppMessage(AppMessage::IconLoaded(key, res)) => {
                Self::handle_icon_loaded(self, key, res)
            }
            ModuleMessage::ActivatedIndex(i) => {
                Self::run_app_at_index(self, i);
                iced::exit()
            }
            ModuleMessage::SelectionUp => {
                if self.selected_index >= 1 {
                    self.selected_index -= 1
                }
                Task::none()
            }
            ModuleMessage::SelectionDown => {
                if self.selected_index + 1 < self.app_list.len() {
                    self.selected_index += 1
                }
                Task::none()
            }
            x => {
                log::warn!("App module received irrelevant msg: {x:?}");
                Task::none()
            }
        }
    }

    fn run(&self) -> Task<crate::message::Message> {
        Self::run_app_at_index(self, self.selected_index);
        iced::exit()
    }
}

async fn get_icon(icon_name: String) -> Option<(String, iced::widget::image::Handle)> {
    let start = iced::debug::time("GetIconTime");

    let final_path = if let Some(icon_path) = ICON_CACHE
        .lock()
        .expect("Can unlock")
        .get(&icon_name)
        .cloned()
    {
        log::debug!("Cache hit! name: {icon_name}");

        icon_path.clone().into()
    } else {
        log::debug!("Cache miss! name: {icon_name}");

        let copy = icon_name.clone();

        if let Some(path) = &APP_SEARCHER.load_icon_path(copy) {
            log::trace!("Icon path from app_searcher with name {icon_name} is {path:?}");

            ICON_CACHE.lock().expect("Can unlock").insert(
                icon_name.to_owned(),
                path.clone().to_str().expect("invalid unicode").to_owned(),
            );
            path.to_path_buf()
        } else {
            PathBuf::new()
        }
    };

    let final_image_handle = APP_SEARCHER.load_icon_image(final_path.as_path())?;

    start.finish();

    if final_path == PathBuf::new() {
        None
    } else {
        Some((final_path.to_str()?.to_string(), final_image_handle))
    }
}
