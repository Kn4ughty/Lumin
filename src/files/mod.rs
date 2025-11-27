use iced::{Task, widget};
use std::sync::mpsc;
use std::thread;
use walkdir::{DirEntry, WalkDir};

use std::path::PathBuf;
// use std::sync::LazyLock;
// use std::sync::Mutex;

use crate::{
    config, constants,
    module::{Module, ModuleMessage},
    widglets,
};

pub struct FileSearcher {
    found_files: Vec<(PathBuf, PathBuf)>,
    reciever: mpsc::Receiver<DirEntry>,
    selected_index: usize,
}

impl Default for FileSearcher {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSearcher {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
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
                    tx.send(entry).expect("Can send");
                    count += 1;
                }
                log::info!("Time to **Send** {count} files: {:#?}", start.elapsed());
            }
        });

        Self {
            found_files: Vec::new(),
            reciever: rx,
            selected_index: 0,
        }
    }
}

impl Module for FileSearcher {
    fn description(&self) -> String {
        String::from("File searching")
    }

    fn view(&self) -> iced::Element<'_, ModuleMessage> {
        let first_few_files: Vec<(PathBuf, PathBuf)> =
            self.found_files.get(0..5).expect("enough files").to_vec();
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
        let start = std::time::Instant::now();
        while let Ok(entry) = self.reciever.try_recv() {
            self.found_files
                .push((entry.file_name().into(), entry.path().to_path_buf()));
        }
        log::trace!("Time to read all the files: {:#?}", start.elapsed());

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
            unknown => log::info!("unknown message {unknown:#?}"),
        }

        Task::none()
    }

    fn run(&self) -> iced::Task<crate::message::Message> {
        self.run_at_index(self.selected_index);
        iced::exit()
    }
}

impl FileSearcher {
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
