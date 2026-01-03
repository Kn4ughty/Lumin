use iced::{Task, advanced::image, widget};
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
    thread,
};
use walkdir::{DirEntry, WalkDir};

use futures::channel::mpsc;
use std::path::PathBuf;

use crate::{
    config, constants,
    module::{Module, ModuleMessage},
    widglets,
};

static ICON_SEARCHER: LazyLock<icon::Icons> = LazyLock::new(icon::Icons::new);
static ICON_LOOKUP: Mutex<LazyLock<HashMap<String, Option<image::Handle>>>> =
    Mutex::new(LazyLock::new(HashMap::new));

#[derive(Debug, Clone)]
pub enum FileMsg {
    FoundFile((PathBuf, Option<image::Handle>)),
}

pub struct FileSearcher {
    /// File name, full path
    found_files: Vec<(PathBuf, Option<image::Handle>)>,
    selected_index: usize,
    have_searched_files: bool,
    start: std::time::Instant,
    // icon_searcher: icon::Icons,
}

impl Default for FileSearcher {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSearcher {
    pub fn new() -> Self {
        Self {
            found_files: Vec::new(),
            selected_index: 0,
            have_searched_files: false,
            start: std::time::Instant::now(),
        }
    }
}

fn file_ext_to_icon_name(ext: &str) -> String {
    // TODO. use /usr/share/mime/globs
    // This also seems relevant
    // https://www.freedesktop.org/wiki/Specifications/shared-mime-info-spec/
    // https://specifications.freedesktop.org/shared-mime-info/0.21/ar01s02.html
    // Steps
    // 0. Read MIME/mime.cache
    //      The spec is sparse on how to do this
    // 1. Find mime type for file in MIME/globs2
    // 2. With mime-type lookup icon for that mimetype in MIME/icons
    //
    //
    match ext {
        "png" | "jpeg" | "svg" | "jpg" | "gif" | "webp" => "image-png",
        "pdf" => "application-pdf",
        "docx" => "application-wps-office.docx",
        "pptx" => "application-wps-office.pptx",
        "mp3" => "audio-mp3",
        "ogg" => "application-ogg",
        "json" => "application-json",
        "md" => "text-markdown",
        "txt" => "text-plain",
        "mp4" | "mkv" | "mov" => "video-mp4",
        _ => "",
    }
    .into()
}

impl Module for FileSearcher {
    fn view(&self) -> iced::Element<'_, ModuleMessage> {
        let first_few_files = match self.found_files.get(0..5) {
            None => self.found_files.clone(),
            Some(slice) => slice.to_vec(),
        };

        widget::scrollable(widget::column(first_few_files.into_iter().enumerate().map(
            |(i, (path, handle))| {
                // log::debug!("icon for path {path:?} found_handle: {handle:?}");

                widglets::ListRow::new(
                    path.file_name()
                        .expect("should not have '..' file")
                        .to_string_lossy(),
                )
                .subtext(path.to_string_lossy())
                .optional_icon(handle)
                .on_activate(ModuleMessage::ActivatedIndex(i))
                .selected(self.selected_index == i)
                .into()
            },
        )))
        .direction(widget::scrollable::Direction::Vertical(
            widget::scrollable::Scrollbar::hidden(),
        ))
        .into()
    }

    fn update(&mut self, msg: ModuleMessage) -> Task<ModuleMessage> {
        match msg {
            ModuleMessage::TextChanged(t) => {
                self.found_files.sort_by_cached_key(|(name, _)| {
                    -crate::sorting::score_element(&t, &name.to_string_lossy())
                });
            }
            ModuleMessage::SelectionUp => {
                if self.selected_index >= 1 {
                    self.selected_index -= 1
                }
            }
            ModuleMessage::SelectionDown => {
                if self.selected_index + 1 < self.found_files.len() {
                    self.selected_index += 1
                }
            }
            ModuleMessage::ActivatedIndex(i) => {
                self.run_at_index(i);
                return iced::exit();
            }
            ModuleMessage::FileMessage(FileMsg::FoundFile(f)) => {
                self.found_files.push(f);
                // log::debug!("time elapsed{}")
            }
            unknown => log::info!("unknown message {unknown:#?}"),
        }

        if self.have_searched_files {
            return Task::none();
        }

        self.have_searched_files = true;
        self.start = std::time::Instant::now();

        // Doing it as a task with streaming many many calls to update which is slow,
        // so a seperate thread is used instead
        Task::run(Self::spawn_file_finder(), |f| f)
            .map(|d| ModuleMessage::FileMessage(FileMsg::FoundFile(d)))
    }

    fn run(&self) -> iced::Task<crate::message::Message> {
        self.run_at_index(self.selected_index);
        iced::exit()
    }
}

impl FileSearcher {
    fn get_data(path: DirEntry) -> (PathBuf, Option<image::Handle>) {
        let path = path.path().to_path_buf();

        let icon_name = &file_ext_to_icon_name(
            &path
                .extension()
                .unwrap_or(std::ffi::OsStr::new(""))
                .to_string_lossy(),
        );

        if let Some(handle) = ICON_LOOKUP.lock().expect("unlock mutex").get(icon_name) {
            return (path, handle.clone());
        }

        let icon_path = ICON_SEARCHER.find_icon(icon_name, 32, 1, "breeze");

        let icon_handle = icon_path.map(|i| {
            if i.path().extension() == Some(std::ffi::OsStr::new("svg")) {
                widglets::svg_path_to_handle(i.path().to_path_buf()).expect("can render svg")
            } else {
                iced::advanced::image::Handle::from_path(i.path())
            }
        });

        ICON_LOOKUP
            .lock()
            .expect("unlock mutex")
            .insert(icon_name.to_string(), icon_handle.clone());

        (path, icon_handle)
    }

    fn spawn_file_finder() -> mpsc::Receiver<(PathBuf, Option<image::Handle>)> {
        let (mut tx, rx) = mpsc::channel(900000);
        thread::spawn(move || -> () {
            let start = std::time::Instant::now();
            let mut count = 0;
            for dir in &config::SETTINGS
                .lock()
                .expect("mutex")
                .file_settings
                .search_directories
            {
                for entry in WalkDir::new(
                    std::sync::LazyLock::force(&constants::HOME_DIR).to_owned() + "/" + dir,
                )
                .into_iter()
                .filter_map(|e| e.ok())
                {
                    tx.try_send(Self::get_data(entry)).expect("Can send");

                    count += 1;
                }
            }
            log::info!("Time to **Send** {count} files: {:#?}", start.elapsed());
        });
        rx
    }
    fn run_at_index(&self, i: usize) {
        Self::open_file(self.found_files[i].0.as_os_str())
    }

    fn open_file(file: &std::ffi::OsStr) {
        let text: &str = if cfg!(target_os = "linux") {
            "xdg-open"
        } else if cfg!(target_os = "macos") {
            "open"
        } else {
            panic!("Unknown operating system")
        };
        crate::util::execute_command_detached(text, vec![file], None).expect("Can launch url")
    }
}
