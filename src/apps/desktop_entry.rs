// https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html
#![allow(dead_code, reason = "Compile time importing shennanigans")]

use std::{collections::HashMap, path::PathBuf, sync::LazyLock, vec::Vec};

use icon;
use walkdir::WalkDir;

use super::{App, Icon};

static ICON_SEARCHER: LazyLock<icon::Icons> = LazyLock::new(icon::Icons::new);

#[derive(Debug, PartialEq)]
pub struct Action {
    pub name: String,
    pub icon_path: Option<String>,
    pub exec: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum EntryType {
    Application,
    Link,
    Directory,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct DesktopEntry {
    pub entry_type: EntryType,
    pub version: Option<String>,
    pub name: String,
    pub generic_name: Option<String>,
    pub comment: Option<String>,
    pub icon: Option<String>,
    pub only_show_in: Vec<String>,
    pub not_show_in: Vec<String>,
    pub try_exec: Option<String>,
    pub exec: String, // Techicially optional, nuh uh.
    pub working_dir: Option<String>,
    pub terminal: bool,
    pub action_list: Vec<Action>,
    pub categories: Vec<String>,
    pub keywords: Vec<String>,
    pub url: Option<String>,
}

impl std::default::Default for DesktopEntry {
    fn default() -> DesktopEntry {
        DesktopEntry {
            entry_type: EntryType::Application,
            version: None,
            name: "".to_string(),
            generic_name: None,
            comment: None,
            icon: None,
            only_show_in: Vec::new(),
            not_show_in: Vec::new(),
            try_exec: None,
            exec: "".to_string(),
            working_dir: None,
            terminal: false,
            action_list: Vec::new(),
            categories: Vec::new(),
            keywords: Vec::new(),
            url: None,
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    MissingRequiredField,
    BadGroupHeader,
    CouldNotLoadFile,
    DesktopEntryHeaderNotFound,
    UnknownApplicationType,
    NoDisplayTrue,
    ActionMissingName,
    MissingDataDirsEnvVar,
}
pub fn get_apps() -> Vec<App> {
    load_desktop_entries()
        .expect("Can load apps")
        .into_iter()
        .map(App::from)
        .collect()
}

pub fn load_desktop_entries() -> Result<Vec<DesktopEntry>, ParseError> {
    let mut entries = Vec::new();
    let Ok(raw_data_dirs) = std::env::var("XDG_DATA_DIRS") else {
        return Err(ParseError::MissingDataDirsEnvVar);
    };
    log::trace!("raw data dirs = {raw_data_dirs}");

    let mut dir_count = 0;

    for dir in raw_data_dirs.split(":") {
        dir_count += 1;

        let mut file_count = 0;

        for entry in WalkDir::new(dir.to_owned() + "/applications/")
            .into_iter()
            .filter_map(|e| e.ok())
        {
            file_count += 1;

            log::trace!("{}", entry.path().display());

            entries.push(parse_from_file(entry.path()).map_err(|e| {
                log::trace!("error parsing file {:#?} with error: {:?}", entry.path(), e)
            }));
        }

        log::debug!("file_count for dir: {dir}, {file_count}");
    }

    log::debug!("{dir_count:#?}");

    Ok(entries.into_iter().filter_map(|a| a.ok()).collect())
}

fn parse_from_file(file_path: &std::path::Path) -> Result<DesktopEntry, ParseError> {
    let contents = std::fs::read_to_string(file_path).map_err(|_| ParseError::CouldNotLoadFile)?;

    parse_from_hashmap(parse_entry_from_string(&contents)?)
}

// Using lifetimes here may look ugly, but it leads to a 30% performance improvement from reduced
// heap allocations
fn parse_entry_from_string<'a>(
    input: &'a str,
) -> Result<HashMap<&'a str, HashMap<&'a str, &'a str>>, ParseError> {
    let mut main_map: HashMap<&'a str, HashMap<&'a str, &'a str>> = HashMap::new();

    let mut current_heading = "";
    let mut current_map: HashMap<&'a str, &'a str> = HashMap::new();

    for line in input.lines() {
        log::trace!("current_line: {line}");

        if let Some(l) = line.split_once('=') {
            current_map.insert(l.0, l.1);
            continue;
        }

        if line.starts_with("#") {
            continue;
        }

        if line.starts_with("[") {
            if !current_map.is_empty() {
                log::trace!("current map was not empty");
                main_map.insert(
                    std::mem::take(&mut current_heading),
                    std::mem::take(&mut current_map),
                );
            }

            current_heading = line
                .get(1..line.len() - 1)
                .ok_or(ParseError::BadGroupHeader)?;
            log::trace!("current heading being set. Is set to {current_heading}");
            continue;
        }
    }
    if !current_map.is_empty() {
        main_map.insert(
            std::mem::take(&mut current_heading),
            std::mem::take(&mut current_map),
        );
    }

    Ok(main_map)
}

#[test]
fn can_parse_entry_from_str() {
    let mut hash = HashMap::new();
    let mut main_map = HashMap::new();
    main_map.insert("Type", "Application");
    main_map.insert("Categories", "System;TerminalEmulator;");
    hash.insert("Desktop Entry", main_map);

    assert_eq!(
        parse_entry_from_string(
            r#"[Desktop Entry]
Type=Application
Categories=System;TerminalEmulator;"#
        )
        .unwrap(),
        hash
    )
}

fn parse_from_hashmap<'a>(
    input: HashMap<&'a str, HashMap<&'a str, &'a str>>,
) -> Result<DesktopEntry, ParseError> {
    let Some(entry_keys) = input.get("Desktop Entry") else {
        return Err(ParseError::DesktopEntryHeaderNotFound);
    };

    if matches!(entry_keys.get("NoDisplay").copied(), Some("true"))
        || matches!(entry_keys.get("Hidden").copied(), Some("true"))
    {
        return Err(ParseError::NoDisplayTrue);
    }

    let entry_type = match entry_keys.get("Type").copied() {
        Some("Application") => EntryType::Application,
        Some("Link") => EntryType::Link,
        Some("Directory") => EntryType::Directory,
        Some(_) => return Err(ParseError::UnknownApplicationType),
        None => return Err(ParseError::MissingRequiredField),
    };

    let entry = DesktopEntry {
        entry_type,
        version: entry_keys.get("Version").map(|s| s.to_string()),
        name: entry_keys // TODO. handle different languages
            .get("Name")
            .ok_or(ParseError::MissingRequiredField)?
            .to_string(),
        try_exec: entry_keys.get("TryExec").map(|s| s.to_string()),
        exec: parse_exec_key(
            entry_keys
                .get("Exec")
                .ok_or(ParseError::MissingRequiredField)?,
            entry_keys.get("Icon").copied(),
            entry_keys.get("Name").copied(),
        ),
        generic_name: entry_keys.get("GenericName").map(|s| s.to_string()),
        comment: entry_keys.get("Comment").map(|s| s.to_string()),
        icon: { entry_keys.get("Icon").map(|s| s.to_string()) },
        only_show_in: parse_string_list(entry_keys.get("OnlyShowIn").copied()),
        not_show_in: parse_string_list(entry_keys.get("NotShowIn").copied()),
        working_dir: entry_keys.get("Path").map(|s| s.to_string()),
        terminal: entry_keys.get("Terminal").is_some_and(|b| *b == "true"),
        categories: parse_string_list(entry_keys.get("Categories").copied()),
        keywords: parse_string_list(entry_keys.get("Keywords").copied()),
        url: match entry_keys.get("URL") {
            Some(s) => Some(s.to_string()),
            _ if entry_type == EntryType::Link => return Err(ParseError::MissingRequiredField),
            _ => None,
        },
        action_list: parse_string_list(entry_keys.get("Actions").copied()) // i dont like this whole thing
            .into_iter()
            .map(|name: String| {
                let formatted_name = &format!("Desktop Action {}", name);
                let section = input
                    .get(formatted_name.as_str())
                    .ok_or(ParseError::BadGroupHeader)?;
                Ok::<Action, ParseError>(Action {
                    name: section
                        .get("Name")
                        .ok_or(ParseError::ActionMissingName)?
                        .to_string(),
                    exec: section.get("Exec").map(|s| s.to_string()),
                    icon_path: section.get("Icon").map(|s| s.to_string()),
                })
            })
            .filter_map(|a| {
                if a.is_ok() {
                    a.ok()
                } else {
                    log::warn!("Action was invalid {:#?}", a.err());
                    None
                }
            })
            .collect(),
    };

    Ok(entry)
}

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
            icon: desktop_entry.icon.map(Icon::NotFoundYet),
        }
    }
}

pub fn load_icon(s: String) -> Option<PathBuf> {
    ICON_SEARCHER
        .find_icon(s.as_str(), 64, 1, "Adwaita")
        .map(|i| i.path) // TODO. Dont hardcode theme
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
        args: ["run", "--branch=stable", "net.ankiweb.Anki", "@@", "@@"]
            .iter()
            .map(|k| k.to_string())
            .collect(),
        working_dir: Some("/".to_string()),
        subname: None,
        icon: None,
    };

    assert_eq!(app, App::from(entry));
}

fn parse_string_list(input: Option<&str>) -> Vec<String> {
    // input is like blah;thing2;thing3
    let mut result = Vec::new();
    let mut current = String::new();
    let chars = input.unwrap_or("").chars().peekable();

    for c in chars {
        match c {
            ';' => {
                if current.ends_with("\\") {
                    current.pop();
                    current.push(c);
                } else {
                    result.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        result.push(current);
    }
    result
}

#[test]
fn can_parse_string_list() {
    let input = Some("t1;t2;t\\;3;");
    let output = parse_string_list(input);
    println!("{output:#?}");
    debug_assert!(output == vec!["t1".to_string(), "t2".to_string(), "t;3".to_string()])
}

fn parse_exec_key(input: &str, icon: Option<&str>, name: Option<&str>) -> String {
    // https://specifications.freedesktop.org/desktop-entry-spec/latest/exec-variables.html
    let mut escaped_result = "".to_string();
    let mut chars = input.chars().peekable();

    // Lots of nesting here..
    while let Some(c) = chars.next() {
        match c {
            // literal \
            '\\' => {
                let Some(n) = chars.next() else {
                    log::warn!("No character after backslash in escape sequence {}", input);
                    continue;
                };

                match n {
                    // Quoting must be done by enclosing the argument between double quotes
                    // and escaping the double quote character , ("`"), ("$"), ("\") by preceding it with an additional backslash character
                    '\\' | '`' | '"' | '$' | '%' => escaped_result.push(n),

                    s => log::info!("unknown escape sequence \\{}. Ignoring.", s),
                }
            }
            // a literal % is escaped as %%
            '%' => {
                let Some(n) = chars.next() else {
                    log::warn!("No character after percentage in escape sequence {}", input);
                    continue;
                };
                match n {
                    '%' => escaped_result.push(n),
                    'i' => {
                        if let Some(icon) = icon {
                            log::debug!(
                                "Hit funny %i (sub in icon) escape sequence for app with icon {icon}"
                            );
                            escaped_result += format!("--icon {icon}").as_str();
                        };
                    }
                    'c' => {
                        if let Some(name) = name {
                            log::debug!(
                                "Hit funny %c (sub in name) escape sequence for app with name {name}"
                            );
                            escaped_result += name;
                        }
                    }
                    _ => {} // do nothing
                }
            }
            _ => escaped_result.push(c),
        }
    }

    escaped_result
}

#[test]
fn can_parse_exec_key() {
    assert_eq!(parse_exec_key(r#"\\"#, None, None), "\\".to_string());
    assert_eq!(parse_exec_key(r#"\`"#, None, None), "`".to_string());
    assert_eq!(parse_exec_key(r#"\%"#, None, None), "%".to_string());
    // Test % escaping
    assert_eq!(
        parse_exec_key(r#"%i"#, Some("icontext"), None),
        "--icon icontext".to_string()
    );
    assert_eq!(
        parse_exec_key(r#"%c"#, None, Some("name")),
        "name".to_string()
    );
}

#[test]
fn can_parse_full_app() {
    let test = r#"
[Desktop Entry]
Type=Application
TryExec=execme
Name=Test Name
Exec=alacritty
Icon=Alacritty
Terminal=false
Categories=System;TerminalEmulator;
GenericName=Terminal
Comment=A fast, cross-platform, OpenGL terminal emulator
Actions=New;

[Desktop Action New]
Name=New Terminal
Exec=testaction
    "#;

    let entry = parse_from_hashmap(parse_entry_from_string(test).unwrap()).unwrap();

    assert_eq!(entry.name, "Test Name");
    assert_eq!(entry.entry_type, EntryType::Application);
    assert_eq!(
        entry.categories,
        vec!["System".to_string(), "TerminalEmulator".to_string()]
    );
    assert_eq!(
        entry.action_list[0],
        Action {
            name: "New Terminal".to_string(),
            exec: Some("testaction".to_string()),
            icon_path: None
        }
    );
}
