use iced::advanced::image;

mod desktop_entry;
use desktop_entry::DesktopEntry;
use libc;
use log;
use std::io;
use std::process;

#[derive(Clone)]
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
        let mut cmd = process::Command::new(self.cmd.clone());
        unsafe {
            cmd.args(self.args.clone())
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
            log::info!("Executing app {:#?}", cmd);
            cmd.spawn()?;
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

                let mut arg: Vec<String> = args.split(" ").map(|s| s.to_string()).collect();

                log::trace!("arg is: {:#?}", arg);

                // There has got to be something wrong here
                // Surely there is a better way
                if arg == vec!["".to_string()] {
                    log::trace!("ARGS LEN 0");
                    arg.clear();
                }

                (
                    cmd.to_string(),
                    arg
                )
            }
            None => (value.exec, vec!["".to_string()]),
        };

        let working_dir = value.working_dir.unwrap_or(
            std::env::var("HOME").unwrap_or("/".to_string())
        );

        App {
            name: value.name,
            cmd,
            args,
            working_dir
        }
    }
}
