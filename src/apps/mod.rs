use std::collections::HashMap;

use iced::Task;
use iced::widget;

#[cfg(target_os = "linux")]
mod desktop_entry;
#[cfg(target_os = "linux")]
use desktop_entry::DesktopEntry;

#[cfg(target_os = "macos")]
mod mac_apps;
use crate::constants;
use crate::module::{Module, ModuleMessage};
use crate::serworse;
use crate::util;
use crate::widglets;

const APP_FREQUENCY_LOOKUP_RELPATH: &str = "app_lookup";

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
        let home = std::env::var("HOME").unwrap();
        let path_string = home + constants::DATA_DIR + APP_FREQUENCY_LOOKUP_RELPATH;
        let path = std::path::Path::new(&path_string);

        let mut map: HashMap<String, u32> = HashMap::new();
        if let Ok(data) = std::fs::read_to_string(path) {
            match serworse::parse_csv::<u32>(&data) {
                Ok(map1) => map = map1,
                Err(e) => log::error!("Could not read app_frequencies to hashmap. E: {e:#?}"),
            }
        } else {
            // Only a warning since this could be the first time the file is created
            log::warn!("Could not read app_frequencies to string");
        };

        AppModule {
            app_list: Vec::new(),
            app_frequencies: map,
        }
    }

    fn run_app_at_index(&self, index: usize) {
        let first = self
            .app_list
            .get(index)
            .expect("selected to run valid index");

        // Increment app frequency hashmap
        let mut map = self.app_frequencies.clone();
        if let Some(old_val) = map.get(&first.name) {
            map.insert(first.name.clone(), *old_val + 1);
        } else {
            map.insert(first.name.clone(), 1);
        }

        log::debug!("New app_frequencies hashmap is {map:#?}");

        let home = std::env::var("HOME").unwrap();
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
        .unwrap();
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
                            None,
                        )
                        .into()
                    }),
            )
            .width(iced::Fill),
        )
        .into()
    }

    fn update(&mut self, msg: ModuleMessage) -> Task<ModuleMessage> {
        // let ModuleMessage::TextChanged(input) = msg else {
        //     return Task::none();
        // };
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
                Self::run_app_at_index(&self, i);
                iced::exit()
            }
            x => {
                log::trace!("App module received irrelevant msg: {x:?}");
                Task::none()
            }
        }
    }

    fn run(&self) {
        Self::run_app_at_index(&self, 0);
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct App {
    cmd: String,
    args: Vec<String>,
    working_dir: Option<String>,
    name: String,
    subname: Option<String>,
}

// This is cheeky and might fail lol
#[cfg(target_os = "linux")]
pub fn get_apps() -> Vec<App> {
    desktop_entry::load_desktop_entries()
        .expect("Can load apps")
        .into_iter()
        .map(App::from)
        .collect()
}

#[cfg(target_os = "macos")]
pub fn get_apps() -> Vec<App> {
    let m_apps = mac_apps::load_all_apps();
    m_apps
        .iter()
        .map(|a| App {
            name: a.name.clone(),
            cmd: "open".into(),
            args: vec![a.path.clone()],
            working_dir: None,
        })
        .collect()
}

#[cfg(target_os = "linux")]
impl From<DesktopEntry> for App {
    fn from(desktop_entry: DesktopEntry) -> Self {
        // https://docs.iced.rs/iced/advanced/image/index.html
        log::trace!("{}", desktop_entry.exec.replace(' ', "*"));
        let (cmd, args) = match desktop_entry.exec.split_once(' ') {
            Some((cmd, args)) => {
                let mut arg: Vec<String> = args
                    .split(" ")
                    .map(|s| s.to_string())
                    .filter(|x| !x.is_empty())
                    .collect();

                log::trace!("arg is: {:#?}", arg);

                if arg == vec!["".to_string()] {
                    log::trace!("ARGS LEN 0");
                    arg.clear();
                }

                (cmd.to_string(), arg)
            }
            None => (desktop_entry.exec, vec!["".to_string()]),
        };

        let working_dir = desktop_entry.working_dir;

        App {
            name: desktop_entry.name,
            cmd,
            args,
            working_dir,
            subname: desktop_entry.generic_name,
        }
    }
}

#[test]
fn can_parse_app_from_desktop_entry() {
    let entry = DesktopEntry {
        name: "anki".to_string(),
        exec: "/usr/bin/flatpak run --branch=stable net.ankiweb.Anki @@ @@".to_string(),
        working_dir: Some("/".to_string()),
        ..Default::default()
    };
    let app = App {
        name: "anki".to_string(),
        cmd: "/usr/bin/flatpak".to_string(),
        args: vec!["run", "--branch=stable", "net.ankiweb.Anki", "@@", "@@"]
            .iter()
            .map(|k| k.to_string())
            .collect(),
        working_dir: Some("/".to_string()),
        subname: None,
    };

    assert_eq!(app, App::from(entry));
}
