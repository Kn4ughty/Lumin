use iced::{Element, Task, Widget, widget};
use numbat::pretty_print::PrettyPrint;
use numbat::{self as nb, Context};

use crate::module::{Module, ModuleMessage};

use crate::widglets;

enum CalcResult<'a> {
    Answer {
        statement: Vec<nb::Statement<'a>>,
        result: nb::InterpreterResult,
    },
    Err(Box<numbat::NumbatError>),
}

pub struct Calc {
    ctx: nb::Context,
    main_text: String,
    sub_text: String,
}

impl Default for Calc {
    fn default() -> Self {
        Self::new()
    }
}

impl Calc {
    pub fn new() -> Self {
        let mut ctx = Context::new(nb::module_importer::BuiltinModuleImporter::default());
        let _ = ctx
            .interpret("use prelude", nb::resolver::CodeSource::Internal)
            .expect("Failed to load numbat prelude"); // TODO. Fail gracefully :p
        Calc {
            ctx,
            main_text: "".to_string(),
            sub_text: "".to_string(),
        }
    }
}

impl Module for Calc {
    fn view(&self) -> Element<'_, ModuleMessage> {
        let font = iced::Font::MONOSPACE;

        widget::column![
            widget::container(
                widglets::heading(widglets::HeadingLevel::H1, self.main_text.clone(), None)
                    .font(font)
            )
            .center_x(iced::Fill),
            widglets::heading(
                widglets::HeadingLevel::Subheading,
                self.sub_text.clone(),
                None
            )
            .font(font),
        ]
        .boxed()
    }

    fn update(&mut self, msg: ModuleMessage) -> Task<ModuleMessage> {
        let ModuleMessage::TextChanged(input) = msg else {
            return Task::none();
        };

        let start = std::time::Instant::now();
        let res = self.calculate_str(&input);
        (self.main_text, self.sub_text) = self.format_result(res);
        log::debug!("Time to calculate calculator was: {:#?}", start.elapsed());
        Task::none()
    }

    fn run(&self) -> Task<crate::message::Message> {
        // Ideally it would save the result to the system clipboard.
        // However, wayland clipbards are stupid so the clipboard data lives only as long as the
        // source window for the data. And then since lumin closes right after, it is lost.
        // Should work if the user has an external clipboard manager however.
        // Maybe i could build a clipboard manager into lumin, like how macos does it.
        // Anyway for now, just do nothing.

        iced::clipboard::write(self.main_text.to_string())
            .map(crate::message::Message::Clipboard)
            .chain(Task::perform(std::future::ready(()), |_| {
                // little stupid hack to make it wait for a moment so the clipboard manager can copy it
                std::thread::sleep(std::time::Duration::from_millis(1));
                crate::message::Message::Close
            }))
    }
}

impl Calc {
    fn calculate_str<'a>(&mut self, input: &'a str) -> CalcResult<'a> {
        match self.ctx.interpret(input, nb::resolver::CodeSource::Text) {
            // Ok((statement, result)) => Answer { statement, result },
            Ok((statement, result)) => CalcResult::Answer { statement, result },

            Err(e) => {
                // idk man
                log::warn!("Calc interpret error: {e:?}");
                // self.ctx.print_diagnostic(error, colorize);
                CalcResult::Err(e)
            }
        }
    }

    /// Returns main text and subtext
    #[must_use]
    fn format_result<'a>(&self, res: CalcResult<'a>) -> (String, String) {
        match res {
            CalcResult::Answer { statement, result } => (
                format!(
                    "{}",
                    result.to_markup(
                        None,
                        self.ctx.dimension_registry(),
                        true,
                        true,
                        &nb::FormatOptions::default()
                    )
                )
                .to_string(),
                {
                    let mut out = String::new();
                    for s in statement {
                        out += &format!("{}", s.pretty_print());
                    }
                    out
                },
            ),
            CalcResult::Err(e) => ("Error".to_string(), format!("{e:?}")),
        }
    }
}
