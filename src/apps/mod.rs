use iced::widget;
use libc;
use log;
use std::io;
use std::process;

mod desktop_entry;
use crate::module::Module;
use crate::util;
use desktop_entry::DesktopEntry;

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
    fn view(&self) -> iced::Element<'_, String> {
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

    fn update(&mut self, input: &str) {
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
    }

    fn run(&self) {
        self.app_list.first().unwrap().execute().unwrap()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct App {
    cmd: String,
    args: Vec<String>,
    working_dir: String,
    pub name: String,
}

#[cfg(unix)]
pub fn get_apps() -> Vec<App> {
    return desktop_entry::load_desktop_entries()
        .expect("Can load apps")
        .into_iter()
        .map(|a| App::from(a))
        .collect();
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

impl App {
    #[cfg(unix)]
    pub fn execute(&self) -> io::Result<()> {
        use std::os::unix::process::CommandExt;

        log::trace!("Execute function being run on app: {self:#?}");

        let mut command = process::Command::new(self.cmd.clone());
        unsafe {
            command
                .args(self.args.clone())
                .current_dir(self.working_dir.clone())
                .stdin(process::Stdio::null())
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null())
                .pre_exec(|| {
                    if libc::setsid() == -1 {
                        return Err(io::Error::last_os_error());
                    }
                    if libc::signal(libc::SIGHUP, libc::SIG_IGN) == libc::SIG_ERR {
                        return Err(io::Error::last_os_error());
                    }
                    Ok(())
                });
            log::info!("Executing app {:#?}", command);
            command.spawn()?;
        }
        Ok(())
    }
}

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

                // There has got to be something wrong here
                // Surely there is a better way
                if arg == vec!["".to_string()] {
                    log::trace!("ARGS LEN 0");
                    arg.clear();
                }

                (cmd.to_string(), arg)
            }
            None => (value.exec, vec!["".to_string()]),
        };

        let working_dir = value
            .working_dir
            .unwrap_or(std::env::var("HOME").unwrap_or("/".to_string()));

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
        working_dir: "/".to_string(),
    };

    assert_eq!(app, App::from(entry));
}
