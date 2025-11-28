use std::fmt::Display;

use anyhow::bail;
use iced::{Element, Task, widget};
use thiserror::Error;

// Some inspiration from here:
// https://docs.rs/crate/calculator-lib/0.1.1/source/src/lib.rs

use crate::module::{Module, ModuleMessage};

use crate::widglets;

const BASE: u32 = 10;

pub struct Calc {
    answer: anyhow::Result<f64>,
}

impl Default for Calc {
    fn default() -> Self {
        Self::new()
    }
}

impl Calc {
    pub fn new() -> Self {
        Calc { answer: Ok(0.0) }
    }
}

impl Module for Calc {
    fn view(&self) -> Element<'_, ModuleMessage> {
        let font = iced::Font::MONOSPACE;

        let widgy = match &self.answer {
            Ok(num) => widget::container(
                widglets::heading(
                    widglets::HeadingLevel::H1,
                    format!("{:#?}", num.clone()),
                    None,
                )
                .font(font),
            )
            .center_x(iced::Fill),
            Err(err) => widget::container(widget::text(err.to_string()).font(font).style(
                |theme: &iced::Theme| widget::text::Style {
                    color: Some(theme.palette().danger),
                },
            )),
        };

        widgy.into()
    }

    fn update(&mut self, msg: ModuleMessage) -> Task<ModuleMessage> {
        let ModuleMessage::TextChanged(input) = msg else {
            return Task::none();
        };

        let start = std::time::Instant::now();
        self.answer = Calc::calculate_str(&input);
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

        if let Ok(number) = self.answer {
            iced::clipboard::write(number.to_string()).chain(Task::perform(
                std::future::ready(()),
                |_| {
                    // little stupid hack to make it wait for a moment so the clipboard manager can copy it
                    std::thread::sleep(std::time::Duration::from_millis(1));
                    crate::message::Message::Close
                },
            ))
        } else {
            Task::none()
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Expr {
    Number(f64),
    Plus,
    Minus,
    UnaryMinus,
    Multiply,
    Divide,
    Power,
    Modulo,
    Bracket(Vec<Expr>),
    OpenParen,
    CloseParen,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::Number(a) => format!("{a}"),
            Self::Plus => String::from("+"),
            Self::Minus => String::from("-"),
            Self::UnaryMinus => String::from("--"),
            Self::Multiply => String::from("*"),
            Self::Divide => String::from("÷"),
            Self::Power => String::from("^"),
            Self::Modulo => String::from("%"),
            Self::Bracket(inner) => inner
                .iter()
                .map(|expr| format!("{} ", expr))
                .collect::<Vec<String>>()
                .concat(),
            Self::OpenParen => String::from("("),
            Self::CloseParen => String::from(")"),
        })
    }
}

impl Expr {
    fn get_number(&self) -> anyhow::Result<f64> {
        if let Self::Number(num) = self {
            Ok(*num)
        } else {
            bail!(
                "Expression was not number! this is strange. Notify the developer with your input message pls"
            )
        }
    }

    fn precedence(&self) -> Option<u8> {
        match self {
            Self::UnaryMinus => Some(4),
            Self::Power => Some(3),
            Self::Divide | Self::Multiply | Self::Modulo => Some(2),
            Self::Plus | Self::Minus => Some(1),
            _ => None,
        }
    }
}

impl Calc {
    pub fn calculate_str(input: &str) -> anyhow::Result<f64> {
        log::trace!("Calculating new input: {input}");
        let tokens = Calc::tokenize(input)?;
        let normalised = Calc::parse_unary_minus(tokens);
        let bracketed = Calc::evaluate_brackets(normalised)?;
        log::trace!("Bracketing finished");
        Calc::calc(bracketed)
    }

    fn tokenize(source: &str) -> anyhow::Result<Vec<Expr>> {
        // TODO. this function sucks. So ugly and buggy and repeated code
        log::trace!("raw tokenize input: {source}");
        let mut out = Vec::new();
        let chars = source.chars();

        let mut number_buf: String = String::new();

        let mut iter = chars.enumerate().peekable();
        while let Some((idx, c)) = iter.next() {
            log::trace!("Character in tokenisation is: {c}");
            if c.is_digit(BASE) || c == '.' {
                log::trace!("Character was num: {c}");
                number_buf.push(c);

                match iter.peek() {
                    Some((_, '.')) | Some((_, '0'..='9')) => {
                        let next = iter.next().expect("Can get next");
                        log::trace!("Next character ({}) was a num or .", next.1);
                        number_buf.push(next.1)
                    }
                    // No number next
                    _ => {
                        log::trace!("Got to num next with: {c}");
                        out.push(Expr::Number(number_buf.parse().map_err(|e| {
                            CalcError::from_expr_list(
                                format!("Tokeniser could not parse to number. Error: {e}"),
                                out.clone(),
                                idx,
                            )
                        })?));
                        number_buf.clear();
                    }
                }
                continue;
            }

            if !number_buf.is_empty() {
                log::trace!("Number buf len was not 0 with {c}");
                out.push(Expr::Number(
                    number_buf.parse().expect("can parse to number"),
                ));
                number_buf.clear();
            }

            log::trace!("Got past number_buf check with char: {c}");

            out.push(match c {
                ' ' => continue,
                '+' => Expr::Plus,
                '-' => Expr::Minus,
                '*' | 'x' => Expr::Multiply,
                '/' | '÷' => Expr::Divide,
                '^' => Expr::Power,
                '%' => Expr::Modulo,
                '(' => Expr::OpenParen,
                ')' => Expr::CloseParen,
                a => {
                    bail!(CalcError::new(
                        format!("Unknown token {a}").to_string(),
                        source.to_string(),
                        idx
                    ))
                }
            });

            log::trace!("Output at end of tokn while loop: {out:?}");
        }

        if !number_buf.is_empty() {
            log::trace!("Number buf len was not 0 at end of function");
            out.push(Expr::Number(number_buf.parse().map_err(|e| {
                CalcError::new(
                    format!("Tokeniser could not parse to number. Error: {e}"),
                    number_buf.to_string(),
                    0,
                )
            })?));
            number_buf.clear();
        }

        Ok(out)
    }

    fn parse_unary_minus(exprs: Vec<Expr>) -> Vec<Expr> {
        let mut out = Vec::new();
        let mut last_was_op = true;

        for e in exprs {
            match e {
                // If current is minus, and previous was operator
                Expr::Minus if last_was_op => out.push(Expr::UnaryMinus),
                _ => out.push(e),
            }
            last_was_op = matches!(
                out.last(),
                Some(
                    Expr::Minus
                        | Expr::Multiply
                        | Expr::Plus
                        | Expr::Power
                        | Expr::Modulo
                        | Expr::Divide
                        | Expr::OpenParen
                )
            );
        }
        out
    }

    fn evaluate_brackets(mut input: Vec<Expr>) -> anyhow::Result<Vec<Expr>> {
        log::trace!("Running evaluate_brackets with input vec: {input:?}");
        let mut equation_buf: Vec<Expr> = vec![]; // Expr inside a set of bracekts
        let mut eval_buf: Vec<Expr> = vec![]; // Expr outside brackets

        let mut bracket_counter = 0;
        let mut bracket_pos: (usize, usize) = (0, 0);

        let mut iter_idx = 0;

        while input.len() > iter_idx {
            let expr = input[iter_idx].clone();
            log::trace!("Current expr = {expr:?}");

            match &expr {
                Expr::OpenParen => {
                    if bracket_counter == 0 {
                        bracket_pos.0 = iter_idx;
                    } else {
                        equation_buf.push(Expr::OpenParen);
                    }

                    bracket_counter += 1;
                }
                Expr::CloseParen => {
                    bracket_counter -= 1;

                    if bracket_counter == 0 {
                        bracket_pos.1 = iter_idx;

                        input.drain(bracket_pos.0..bracket_pos.1);

                        eval_buf.insert(
                            bracket_pos.0,
                            Expr::Bracket(Self::evaluate_brackets(equation_buf.clone())?),
                        );
                        log::trace!("Finished recursion. Eval_buf: {eval_buf:?}");

                        iter_idx = bracket_pos.0;

                        equation_buf.clear();
                    } else {
                        equation_buf.push(Expr::CloseParen);
                    }
                }
                _ => {
                    if bracket_counter > 0 {
                        equation_buf.push(expr.clone())
                    } else {
                        eval_buf.push(expr.clone())
                    }
                }
            }
            iter_idx += 1;
        }

        Ok(eval_buf)
    }

    fn apply_op(input: &mut Vec<Expr>, idx: usize) -> anyhow::Result<()> {
        let op = input[idx].clone();

        match input[idx] {
            Expr::UnaryMinus => {
                let rhs = match input.clone().get(idx + 1).ok_or(CalcError::from_expr_list(
                    "RHS of UnaryMinus not found".to_string(),
                    input.clone(),
                    idx + 1,
                ))? {
                    Expr::Bracket(_) => unreachable!("Should not have brackets at this stage"),
                    Expr::Number(inner) => *inner,
                    _ => bail!(CalcError::from_expr_list(
                        String::from("Could not turn RHS of UnaryMinus into number"),
                        input.clone(),
                        idx + 1,
                    )),
                };
                input.drain(idx..=idx + 1); // Safe since passed above .get()
                input.insert(idx, Expr::Number(-rhs));
            }
            _ => {
                if idx == 0 {
                    bail!(CalcError::from_expr_list(
                        "Operator as first arg isnt allowed".to_string(),
                        input.clone(),
                        idx
                    ))
                }

                let lhs = match input.clone().get(idx - 1).ok_or(CalcError::from_expr_list(
                    "LHS of operator not found. This is a strange state. Pls notify developer"
                        .to_string(),
                    input.clone(),
                    idx,
                ))? {
                    Expr::Bracket(inner) => Self::calc(inner.clone())?,
                    Expr::Number(inner) => *inner,
                    _ => bail!(CalcError::from_expr_list(
                        String::from(
                            "LHS Expression could not be turned into number. This is a strange state. Pls notify developer"
                        ),
                        input.clone(),
                        idx - 1
                    )),
                };

                if input.clone().get(idx + 1).is_none() {
                    input.push(Expr::Number(0.0));
                }

                let rhs = match input.clone().get(idx + 1).ok_or(CalcError::from_expr_list(
                    "RHS of operator not found".to_string(),
                    input.clone(),
                    idx + 1,
                ))? {
                    Expr::Bracket(inner) => Self::calc(inner.clone())?,
                    Expr::Number(inner) => *inner,
                    _ => bail!(CalcError::from_expr_list(
                        String::from("RHS Expression could not be turned into number"),
                        input.clone(),
                        idx + 1
                    )),
                };

                let val = match op {
                    Expr::Divide => lhs / rhs,
                    Expr::Plus => lhs + rhs,
                    Expr::Minus => lhs - rhs,
                    Expr::Power => lhs.powf(rhs),
                    Expr::Modulo => lhs % rhs,
                    Expr::Multiply => lhs * rhs,
                    _ => unreachable!("Should have been an operator"),
                };

                input.drain(idx - 1..=idx + 1);
                input.insert(idx - 1, Expr::Number(val));
            }
        }

        Ok(())
    }

    fn calc(mut input: Vec<Expr>) -> anyhow::Result<f64> {
        // Evaluate brackets
        let mut last_expr: Option<Expr> = None;
        let mut i = 0;
        while input.len() > i {
            if let Expr::Bracket(inner) = &input[i] {
                let val = Self::calc(inner.clone())?;
                input[i] = Expr::Number(val);

                if let Some(Expr::Number(_)) = last_expr {
                    input.insert(i, Expr::Multiply);
                    i += 1
                }
            }
            last_expr = Some(input[i].clone());
            i += 1
        }

        log::trace!("Second pass of calc. Input is now: {:?}", input);

        for prec in (1..=4).rev() {
            log::trace!("for loop entered with prec: {prec}");
            let mut idx = 0;

            while idx < input.len() {
                log::trace!("entered while loop. Prec {prec}. idx: {idx}");

                if input[idx].precedence() == Some(prec) {
                    log::trace!("input before idx: {idx} apply op: {input:?}");
                    Self::apply_op(&mut input, idx)?;
                    log::trace!("input AFTER  idx: {idx}apply op: {input:?}");
                    // Stay on left
                    idx = idx.saturating_sub(1);
                    continue;
                }
                log::trace!(
                    "Precedence did not match. {:?}, prec: {}",
                    input[idx].precedence(),
                    prec
                );
                idx += 1;
            }
        }

        if input.len() != 1 {
            bail!(CalcError::from_expr_list(
                "Invalid expression".to_string(),
                input,
                0,
            ))
        }

        input[0].get_number()
    }
}

#[test]
fn can_convert_source_to_tokenvec() {
    assert_eq!(
        Calc::tokenize("1 + 22-1*2.7 + 12^2").unwrap(),
        vec![
            Expr::Number(1.0),
            Expr::Plus,
            Expr::Number(22.0),
            Expr::Minus,
            Expr::Number(1.0),
            Expr::Multiply,
            Expr::Number(2.7),
            Expr::Plus,
            Expr::Number(12.0),
            Expr::Power,
            Expr::Number(2.0),
        ]
    );
    assert_eq!(
        Calc::tokenize("1+22").unwrap(),
        vec![Expr::Number(1.0), Expr::Plus, Expr::Number(22.0),]
    );
}

#[test]
fn can_parse_unary_minus() {
    assert_eq!(
        Calc::parse_unary_minus(vec![Expr::Minus, Expr::Number(1.0)]),
        vec![Expr::UnaryMinus, Expr::Number(1.0)]
    );

    assert_eq!(
        Calc::parse_unary_minus(vec![Expr::Power, Expr::Minus, Expr::Number(1.0)]),
        vec![Expr::Power, Expr::UnaryMinus, Expr::Number(1.0)]
    );
}

#[test]
fn can_parse_brackets() {
    assert_eq!(
        Calc::evaluate_brackets(Calc::tokenize("((1+4)*2)").unwrap()).unwrap(),
        vec![Expr::Bracket(vec![
            Expr::Bracket(vec![Expr::Number(1.0), Expr::Plus, Expr::Number(4.0)]),
            Expr::Multiply,
            Expr::Number(2.0)
        ])]
    );

    // TODO. Add test for trailing openParen
}

#[test]
fn can_do_math() {
    assert_eq!(Calc::calculate_str("2(2)").unwrap(), 4.0);
    assert_eq!(Calc::calculate_str("2^(1+2)").unwrap(), 8.0);
    assert_eq!(Calc::calculate_str("12 / 2 * 3 ").unwrap(), 18.0);
    assert_eq!(
        Calc::calculate_str("((5^3 + 2) *(6 / 2)) / 3").unwrap(),
        127.0
    );

    assert_eq!(
        Calc::calculate_str("(144/12 + 8^2) - (2^4 * 7) + 3^3 + 0.5").unwrap(),
        -8.5
    );

    assert_eq!(Calc::calculate_str("12^2").unwrap(), 144.0);

    assert_eq!(
        Calc::calculate_str(
            "((5^3 + 4^2) * (12^2 - 6^3)) / (3^2 + 7) + (144/12 + 8^2) - (2^4 * 7) + 3^3 + 0.5"
        )
        .unwrap(),
        -643.0
    );
    // Unary minus
    assert_eq!(Calc::calculate_str("2^-3").unwrap(), 0.125);
}

#[derive(Debug, Error)]
struct CalcError {
    message: String,
    equation: String,
    error_idx: usize,
}

impl CalcError {
    fn new(message: String, equation: String, error_idx: usize) -> Self {
        CalcError {
            message,
            equation,
            error_idx,
        }
    }

    fn from_expr_list(message: String, equation: Vec<Expr>, error_idx: usize) -> Self {
        let mut equation_string = String::new();

        for expr in equation {
            equation_string.push_str(&format!("{} ", expr));
        }
        CalcError {
            message: (message),
            equation: (equation_string),
            error_idx: { if error_idx != 0 { error_idx * 2 + 1 } else { 0 } },
        }
    }
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Calc Error! \nMessage: {}\n{}\n{}",
            self.message,
            self.equation,
            {
                let mut indent = String::new();

                for _ in 0..self.error_idx {
                    indent.push(' ');
                }
                indent.push('^');
                indent
            }
        ))
    }
}
