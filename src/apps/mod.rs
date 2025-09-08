use iced::advanced::image;

mod desktop_entry;
use desktop_entry::DesktopEntry;
use libc;
use std::io;
use std::process;
use log;

#[derive(Clone)]
pub struct App {
    cmd: String,
    args: Vec<String>,
    working_dir: String,
    pub name: String,
}

#[cfg(unix)]
pub fn get_apps() -> Vec<App> {
    return desktop_entry::load_desktop_entries().expect("Can load apps").into_iter().map(|a| App::from(a)).collect();
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
    fn execute_detached(&self) -> io::Result<()> {
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
            cmd.spawn()?;
        }
        Ok(())
    }
}

impl From<DesktopEntry> for App {
    fn from(value: DesktopEntry) -> Self {
        // https://docs.iced.rs/iced/advanced/image/index.html
        let (cmd, args) = match value.exec.split_once(' ') {
            Some((cmd, args)) => (
                cmd.to_string(),
                args.split(" ").map(|s| s.to_string()).collect(),
            ),
            None => (value.exec, vec!["".to_string()]),
        };

        App {
            name: value.name,
            cmd,
            args,
            working_dir: value.working_dir.unwrap_or("/".to_string()), // Should be $HOME
        }
    }
}

