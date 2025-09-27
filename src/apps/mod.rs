use iced::Task;
use iced::widget;
use log;

#[cfg(target_os = "linux")]
mod desktop_entry;
#[cfg(target_os = "linux")]
use desktop_entry::DesktopEntry;

#[cfg(target_os = "macos")]
mod mac_apps;
use crate::module::{Module, ModuleMessage};
use crate::util;

pub struct AppModule {
    app_list: Vec<App>,
}

impl AppModule {
    pub fn new() -> Self {
        AppModule {
            app_list: Vec::new(),
        }
    }
}

impl Module for AppModule {
    fn view(&self) -> iced::Element<'_, ModuleMessage> {
        widget::scrollable(
            widget::column(
                self.app_list
                    .clone()
                    .into_iter()
                    .map(|app| widget::text(app.name).into()),
            )
            .width(iced::Fill),
        )
        .into()
    }

    fn update(&mut self, msg: ModuleMessage) -> Task<ModuleMessage> {
        let ModuleMessage::TextChanged(input) = msg else {
            return Task::none();
        };

        if self.app_list.len() == 0 {
            log::trace!("Regenerating app_list");
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
            // TODO. Add aditional weighting for first character matching
            return score * -1;
        });

        log::debug!(
            "Time to sort #{} apps: {:#?}",
            self.app_list.len(),
            start.elapsed()
        );

        Task::none()
    }

    fn run(&self) {
        let first = self
            .app_list
            .first()
            .expect("There should be at least 1 result");
        util::execute_command_detached(
            first.cmd.clone(),
            first.args.clone(),
            first.working_dir.clone(),
        )
        .unwrap();
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct App {
    cmd: String,
    args: Vec<String>,
    working_dir: Option<String>,
    name: String,
}

#[cfg(target_os = "linux")]
pub fn get_apps() -> Vec<App> {
    return desktop_entry::load_desktop_entries()
        .expect("Can load apps")
        .into_iter()
        .map(|a| App::from(a))
        .collect();
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

// #[test]
// fn becnhapps() {
//     let now = std::time::Instant::now();
//
//     let _a = get_apps();
//
//     println!("time: {:?}", now.elapsed());
//     assert!(1==2);
// }

#[cfg(target_os = "linux")]
impl From<DesktopEntry> for App {
    fn from(value: DesktopEntry) -> Self {
        // https://docs.iced.rs/iced/advanced/image/index.html
        log::trace!("{}", value.exec.replace(' ', "*"));
        let (cmd, args) = match value.exec.split_once(' ') {
            Some((cmd, args)) => {
                let mut arg: Vec<String> = args
                    .split(" ")
                    .map(|s| s.to_string())
                    .filter(|x| x.len() > 0)
                    .collect();

                log::trace!("arg is: {:#?}", arg);

                if arg == vec!["".to_string()] {
                    log::trace!("ARGS LEN 0");
                    arg.clear();
                }

                (cmd.to_string(), arg)
            }
            None => (value.exec, vec!["".to_string()]),
        };

        let working_dir = value.working_dir;

        App {
            name: value.name,
            cmd,
            args,
            working_dir,
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
    };

    assert_eq!(app, App::from(entry));
}
