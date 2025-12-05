use iced::{Task, widget};
use std::thread;
use walkdir::{DirEntry, WalkDir};

use futures::channel::mpsc;
use std::path::PathBuf;

use crate::{
    config, constants,
    module::{Module, ModuleMessage},
    widglets,
};

#[derive(Debug, Clone)]
pub enum FileMsg {
    FoundFile((PathBuf, PathBuf)),
}

pub struct FileSearcher {
    /// File name, full path
    found_files: Vec<(PathBuf, PathBuf)>,
    selected_index: usize,
    have_searched_files: bool,
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
        }
    }
}

impl Module for FileSearcher {
    fn view(&self) -> iced::Element<'_, ModuleMessage> {
        let first_few_files: Vec<(PathBuf, PathBuf)> = match self.found_files.get(0..5) {
            None => self.found_files.clone(),
            Some(slice) => slice.to_vec(),
        };

        widget::scrollable(widget::column(first_few_files.into_iter().enumerate().map(
            |(i, (name, path))| {
                widglets::ListRow::new(name.to_string_lossy())
                    .subtext(path.to_string_lossy())
                    .on_activate(ModuleMessage::ActivatedIndex(i))
                    .selected(self.selected_index == i)
                    .into()
            },
        )))
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
            }
            unknown => log::info!("unknown message {unknown:#?}"),
        }

        if !self.have_searched_files {
            self.have_searched_files = true;

            Task::run(Self::spawn_file_finder(), |f| {
                (f.file_name().into(), f.path().to_path_buf())
            })
            .map(|d| ModuleMessage::FileMessage(FileMsg::FoundFile(d)))
        } else {
            Task::none()
        }
    }

    fn run(&self) -> iced::Task<crate::message::Message> {
        self.run_at_index(self.selected_index);
        iced::exit()
    }
}

impl FileSearcher {
    fn spawn_file_finder() -> mpsc::Receiver<DirEntry> {
        let (mut tx, rx) = mpsc::channel(900000);
        thread::spawn(move || {
            let start = std::time::Instant::now();
            let mut count = 0;
            for dir in &config::SETTINGS.file_settings.search_directories {
                for entry in WalkDir::new(
                    std::sync::LazyLock::force(&constants::HOME_DIR).to_owned() + "/" + dir,
                )
                .into_iter()
                .filter_map(|e| e.ok())
                {
                    tx.try_send(entry).expect("Can send");
                    count += 1;
                }
            }
            log::info!("Time to **Send** {count} files: {:#?}", start.elapsed());
        });
        rx
    }
    fn run_at_index(&self, i: usize) {
        Self::open_file(self.found_files[i].1.as_os_str())
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
