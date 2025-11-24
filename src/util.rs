use std::convert::AsRef;
use std::ffi::OsStr;
use std::io;
use std::process;

#[cfg(unix)]
pub fn execute_command_detached<S, I, A>(
    cmd: S,
    args: I,
    working_dir_arg: Option<String>,
) -> io::Result<()>
where
    S: AsRef<OsStr>,
    A: AsRef<OsStr>,
    I: IntoIterator<Item = A>,
{
    use std::os::unix::process::CommandExt;

    let working_dir: String =
        working_dir_arg.unwrap_or(std::env::var("HOME").unwrap_or("/".into()));

    let mut command = process::Command::new(cmd);
    unsafe {
        command
            .args(args)
            .current_dir(working_dir)
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
