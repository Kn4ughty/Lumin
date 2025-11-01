use std::collections::HashMap;

use iced::Task;
use iced::widget;

pub mod desktop_entry;
pub mod mac_apps;

#[cfg(target_os = "linux")]
use desktop_entry::get_apps;
#[cfg(target_os = "macos")]
use mac_apps::get_apps;

use crate::constants;
use crate::module::{Module, ModuleMessage};
use crate::serworse;
use crate::util;
use crate::widglets;

const APP_FREQUENCY_LOOKUP_RELPATH: &str = "app_lookup";

#[derive(Clone, PartialEq, Debug)]
pub struct App {
    cmd: String,
    icon: Option<widget::image::Handle>,
    args: Vec<String>,
    working_dir: Option<String>,
    name: String,
    subname: Option<String>,
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
    pub fn new() -> Self {
        // attempt to load hashmap from disk
        let home = std::env::var("HOME").expect("Can get home EnvVar");
        let path_string = home + constants::DATA_DIR + APP_FREQUENCY_LOOKUP_RELPATH;
        let path = std::path::Path::new(&path_string);

        let mut map: HashMap<String, u32> = HashMap::new();
        if let Ok(data) = std::fs::read_to_string(path) {
            match serworse::parse_csv::<u32>(&data) {
                Ok(map1) => map = map1,
                Err(e) => log::error!("Could not read app_frequencies to hashmap. E: {e:#?}"),
            }
        } else {
            log::warn!(
                "Could not read app_frequencies to string.\
                Once any app is launched for the first time, \
                this warning should go away as the hashmap should have been written"
            );
        };

        AppModule {
            app_list: Vec::new(),
            app_frequencies: map,
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

        let home = std::env::var("HOME").expect("Can get home EnvVar");
        let path_string = home + constants::DATA_DIR + APP_FREQUENCY_LOOKUP_RELPATH;
        let path = std::path::Path::new(&path_string);

        if let Err(e) = std::fs::write(path, serworse::hash_map_to_csv(map)) {
            log::error!(
                "Could not write new app frequency hashmap to file!! e: {e}\nHashmap is: {e:#?}"
            );
        } else {
            log::trace!("Successfully wrote to path: {path:?}");
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
                        widglets::listrow(
                            app.name,
                            app.subname,
                            Some(ModuleMessage::ActivatedIndex(i)),
                            app.icon,
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
                    self.app_list = get_apps();
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

                Task::none()
            }
            ModuleMessage::ActivatedIndex(i) => {
                Self::run_app_at_index(self, i);
                iced::exit()
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
