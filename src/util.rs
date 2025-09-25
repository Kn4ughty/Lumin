use libc;
use std::convert::AsRef;
use std::ffi::OsStr;
use std::io;
use std::process;

// https://www.geeksforgeeks.org/dsa/longest-common-substring-dp-29/
pub fn longest_common_substr(s1: &str, s2: &str) -> i32 {
    // Stringslice?

    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();

    let m = s1.len(); // This will have problems because of difference of chars and bytes
    let n = s2.len();

    let mut prev = vec![0; n + 1];

    let mut res: i32 = 0;

    for i in 1..m + 1 {
        let mut curr = vec![0; n + 1];
        for j in 1..n + 1 {
            if s1[i - 1] == s2[j - 1] {
                curr[j] = prev[j - 1] + 1;
                res = res.max(curr[j]);
            } else {
                curr[j] = 0;
            }
        }
        prev = curr;
    }

    res
}

#[test]
fn longest_common_substr_correct() {
    assert_eq!(longest_common_substr("hello world", "world"), 5);
    assert_eq!(
        longest_common_substr("geeksforgeeks", "ggeegeeksquizpractice"),
        5
    );
    assert_eq!(longest_common_substr("", ""), 0);
}

#[cfg(unix)]
pub fn execute_command_detached<S, I>(
    cmd: S,
    args: I,
    working_dir_arg: Option<String>,
) -> io::Result<()>
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S>,
{
    use std::os::unix::process::CommandExt;

    // log::trace!("Execute function being run on app: {self:#?}");

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
