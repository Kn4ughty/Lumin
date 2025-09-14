
use anyhow;
use anyhow::bail;
use arboard::Clipboard;
use iced::{Element, widget};
use thiserror::Error;

// This is my fav so far
// https://docs.rs/crate/calculator-lib/0.1.1/source/src/lib.rs
// stealing >:)

use crate::module::Module;
//

const BASE: u32 = 10;

// pub enum CalcMsg {}

pub struct Calc {
    answer: anyhow::Result<f64>,
}

impl Calc {
    pub fn new() -> Self {
        Calc { answer: Ok(0.0) }
    }
}

impl Module for Calc {
    fn view(&self) -> Element<'_, String> {
        let mut font = iced::Font::MONOSPACE;
        font.weight = iced::font::Weight::Bold;

        let widgy = match &self.answer {
            Ok(num) => widget::container(widget::text(format!("{:#?}", num.clone())).font(font))
                .center(iced::Fill),
            Err(err) => widget::container(widget::text(err.to_string()).font(font).style(
                |theme: &iced::Theme| widget::text::Style {
                    color: Some(theme.palette().danger),
                },
            )),
        };

        widgy.into()
    }

    fn update(&mut self, input: &str) {
        let start = std::time::Instant::now();
        self.answer = Calc::calculate_str(input);
        log::debug!("Time to calculate calculator was: {:#?}", start.elapsed());
    }

    fn run(&self) {
        // ToDo. Make own clipboard library
        // let mut clipboard = Clipboard::new().unwrap();
        // clipboard.set_text(self.answer.clone()).unwrap();
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Expr {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Bracket(Vec<Expr>),
    OpenParen,
    CloseParen,
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        // f.write_str("hello")
        // let mut f = String::new();
        let f: String = match self {
            Self::Number(a) => format!("{a}"),
            Self::Plus => String::from("+"),
            Self::Minus => String::from("-"),
            Self::Multiply => String::from("*"),
            Self::Divide => String::from("÷"),
            Self::Power => String::from("^"),
            Self::Bracket(inner) => inner
                .iter()
                .map(|expr| format!("{} ", expr.to_string()))
                .collect::<Vec<String>>()
                .concat(),
            Self::OpenParen => String::from("("),
            Self::CloseParen => String::from(")"),
        };
        f
    }
}

impl Expr {
    fn get_number(&self) -> anyhow::Result<f64> {
        if let Self::Number(num) = self {
            return Ok(*num);
        } else {
            bail!("Expression was not number")
        }
    }
}

//
// impl Module for Calc {
//     fn update(&mut self, input: &str) {}
//     fn view(&self) -> Element<'_, String> {}
//     fn run(&self) {
//         // Copy to clipboard?
//     }
// }

impl Calc {
    fn tokenize(source: &str) -> anyhow::Result<Vec<Expr>> {
        // TODO. this function sucks. So ugly and buggy and repeated code
        log::trace!("raw tokenize input: {source}");
        let mut out = Vec::new();
        let chars = source.chars();

        let mut number_buf: String = "".to_string();

        // TODO. Convert to use enumerate
        let mut iter = chars.enumerate().peekable();
        while let Some((idx, c)) = iter.next() {
            log::trace!("Character in tokenisation is: {c}");
            if c.is_digit(BASE) || c == '.' {
                log::trace!("Character was num: {c}");
                number_buf.push(c);

                match iter.peek() {
                    Some((_, '.')) | Some((_, '0'..='9')) => {
                        let next = iter.next().unwrap();
                        log::trace!("Next character ({}) was a num or .", next.1);
                        number_buf.push(next.1)
                    }
                    // No number next
                    _ => {
                        log::trace!("Got to num next with: {c}");
                        out.push(Expr::Number(number_buf.parse().map_err(|_| {
                            CalcError::from_expr_list(
                                String::from("Could not parse to number"),
                                out.clone(),
                                idx,
                            )
                        })?));
                        number_buf.clear();
                    }
                }
                continue;
            }

            if number_buf.len() != 0 {
                log::trace!("Number buf len was not 0 with {c}");
                out.push(Expr::Number(number_buf.parse().unwrap()));
                number_buf.clear();
            }

            log::trace!("Got past number_buf check with char: {c}");

            out.push(match c {
                ' ' => continue,
                '+' => Expr::Plus,
                '-' => Expr::Minus,
                '*' => Expr::Multiply,
                '/' => Expr::Divide,
                '^' => Expr::Power,
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

        if number_buf.len() != 0 {
            log::trace!("Number buf len was not 0 at end of function");
            out.push(Expr::Number(number_buf.parse().unwrap()));
            number_buf.clear();
        }

        Ok(out)
    }

    fn evaluate_brackets(mut input: Vec<Expr>) -> anyhow::Result<Vec<Expr>> {
        log::trace!("Running evaluate_brackets with input vec: {input:?}");
        // Returns
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

        return Ok(eval_buf);
    }

    fn calc(mut input: Vec<Expr>) -> anyhow::Result<f64> {
        let mut iter_idx = 0;
        let mut last_expr: Option<Expr> = None;

        while input.len() > iter_idx {
            let expr = input[iter_idx].clone();

            log::trace!("1st pass state. i: {iter_idx}, input: {input:?}");

            match expr {
                Expr::Bracket(ref inner) => {
                    // handle inner. (recurse)
                    input.remove(iter_idx);
                    input.insert(iter_idx, Expr::Number(Self::calc(inner.to_vec())?));

                    if matches!(last_expr, Some(Expr::Bracket(_)))
                        || matches!(last_expr, Some(Expr::Number(_)))
                    {
                        input.insert(iter_idx, Expr::Multiply);
                        last_expr = None;
                    }
                    continue;
                }
                Expr::OpenParen => unreachable!(),
                Expr::CloseParen => unreachable!(),
                Expr::Number(_) => (),
                _ => {
                    log::trace!("Character must be operator");

                    if iter_idx == 0 {
                        bail!(CalcError::from_expr_list(
                            "Operator as first character isnt allowed.\n\
                                You need a left hand side and right hand side"
                                .to_string(),
                            input,
                            iter_idx
                        ))
                    }
                    let lhs =
                        match input
                            .clone()
                            .get(iter_idx - 1)
                            .ok_or(CalcError::from_expr_list(
                                "LHS of operator not found".to_string(),
                                input.clone(),
                                iter_idx,
                            ))? {
                            Expr::Bracket(inner) => Self::calc(inner.to_vec())?,
                            Expr::Number(inner) => inner.clone(),
                            _ => bail!(CalcError::from_expr_list(
                                String::from("Expression missing Expression could not be turned into number"),
                                input,
                                iter_idx
                            )),
                        };

                    let rhs =
                        match input
                            .clone()
                            .get(iter_idx + 1)
                            .ok_or(CalcError::from_expr_list(
                                "RHS of operator not found".to_string(),
                                input.clone(),
                                iter_idx,
                            ))? {
                            Expr::Bracket(inner) => Self::calc(inner.to_vec())?,
                            Expr::Number(inner) => inner.clone(),
                            _ => bail!(CalcError::from_expr_list(
                                String::from("Expression could not be turned into number"),
                                input,
                                iter_idx
                            )),
                        };
                    log::trace!("1st pass. lhs: {lhs:?}. rhs: {rhs:?}");

                    match expr {
                        Expr::Multiply => {
                            input.drain(iter_idx - 1..=iter_idx + 1);
                            input.insert(iter_idx - 1, Expr::Number(lhs * rhs));
                            iter_idx -= 1;
                        }
                        Expr::Power => {
                            input.drain(iter_idx - 1..=iter_idx + 1);
                            input.insert(iter_idx - 1, Expr::Number(lhs.powf(rhs)));
                            // input.insert(iter_idx - 1, Expr::Number(rhs.powf(lhs)));
                            iter_idx -= 1;
                        }
                        Expr::Divide => {
                            input.drain(iter_idx - 1..=iter_idx + 1);
                            input.insert(iter_idx - 1, Expr::Number(lhs / rhs));
                            iter_idx -= 1;
                        }
                        _ => (),
                    }
                }
            }
            last_expr = Some(expr);
            iter_idx += 1
        }

        // Begin second pass.
        log::trace!("Second pass of calc. Input is now: {:?}", input);

        iter_idx = 0;
        while input.len() > iter_idx {
            log::trace!("2nd pass state. i: {iter_idx}, input: {input:?}");
            let expr = input[iter_idx].clone();

            match expr {
                Expr::Bracket(_) => unreachable!(),
                Expr::OpenParen => unreachable!(),
                Expr::CloseParen => unreachable!(),
                Expr::Number(_) => (),
                _ => {
                    // Must be operator
                    let lhs =
                        match input
                            .clone()
                            .get(iter_idx - 1)
                            .ok_or(CalcError::from_expr_list(
                                "Expression not found".to_string(),
                                input.clone(),
                                iter_idx,
                            ))? {
                            Expr::Bracket(inner) => Self::calc(inner.to_vec())?, // unreachable??
                            Expr::Number(inner) => inner.clone(),
                            _ => bail!(CalcError::from_expr_list(
                                String::from("Expression could not be turned into number"),
                                input,
                                iter_idx
                            )),
                        };

                    let rhs =
                        match input
                            .clone()
                            .get(iter_idx + 1)
                            .ok_or(CalcError::from_expr_list(
                                "Expression not found".to_string(),
                                input.clone(),
                                iter_idx,
                            ))? {
                            Expr::Bracket(inner) => Self::calc(inner.to_vec())?,
                            Expr::Number(inner) => inner.clone(),
                            _ => bail!(CalcError::from_expr_list(
                                String::from("Expression could not be turned into number"),
                                input,
                                iter_idx
                            )),
                        };

                    match expr {
                        Expr::Plus => {
                            input.drain(iter_idx - 1..=iter_idx + 1);
                            input.insert(iter_idx - 1, Expr::Number(lhs + rhs));
                            iter_idx -= 1;
                        }
                        Expr::Minus => {
                            input.drain(iter_idx - 1..=iter_idx + 1);
                            input.insert(iter_idx - 1, Expr::Number(lhs - rhs));
                            iter_idx -= 1;
                        }
                        a => unreachable!("Got to end with unknown thing {a:?}"),
                    }
                }
            }

            // last_expr = Some(expr);
            iter_idx += 1;
        }

        if input.is_empty() || input.len() != 1 {
            bail!(CalcError::from_expr_list(
                String::from("Empty equation or invalid"),
                input.clone(),
                1,
            ))
        }

        Ok(input[0].get_number()?)
    }


    fn calculate_str(input: &str) -> anyhow::Result<f64> {
        // Calc::calc(Calc::evaluate_brackets(Calc::tokenize(input.to_string()))
        log::trace!("Calculating new input: {input}");
        let tokens = Calc::tokenize(input)?;
        let bracketed = Calc::evaluate_brackets(tokens)?;
        log::trace!("Bracketing finished");
        Calc::calc(bracketed)
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
fn can_parse_brackets() {
    assert_eq!(
        Calc::evaluate_brackets(Calc::tokenize("((1+4)*2)").unwrap()).unwrap(),
        vec![Expr::Bracket(vec![
            Expr::Bracket(vec![Expr::Number(1.0), Expr::Plus, Expr::Number(4.0)]),
            Expr::Multiply,
            Expr::Number(2.0)
        ])]
    );

    // TODO.
    // Add test for trailing openParen
}

#[test]
fn can_do_math() {
    // let input = "((5^3 + 4^2) * (12^2 - 6^3)) / (3^2 + 7) + (144/12 + 8^2) - (2^4 * 7) + 3^3 + 0.5";
    // let input = "2(2)";
    // let answer = Calc::calculate_str(input).unwrap();
    assert_eq!(Calc::calculate_str("2(2)").unwrap(), 4.0);
    assert_eq!(Calc::calculate_str("2^(1+2)").unwrap(), 8.0);
    assert_eq!(Calc::calculate_str("12 / 2 * 3 ").unwrap(), 18.0);
    assert_eq!(
        Calc::calculate_str("((5^3 + 2) *(6 / 2)) / 3").unwrap(),
        127.0
    );

    // Sub tests for big equation
    assert_eq!(
        Calc::calculate_str("(144/12 + 8^2) - (2^4 * 7) + 3^3 + 0.5").unwrap(),
        -8.5
    );

    assert_eq!(Calc::calculate_str("12^2").unwrap(), 144.0);
    // assert_eq!(Calc::calculate_str("1+22").unwrap(), 23.0);

    assert_eq!(
        Calc::calculate_str(
            "((5^3 + 4^2) * (12^2 - 6^3)) / (3^2 + 7) + (144/12 + 8^2) - (2^4 * 7) + 3^3 + 0.5"
        )
        .unwrap(),
        -643.0
    );
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
            equation_string.push_str(&format!("{} ", expr.to_string()).to_string());
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

// TEST THESE
// 4 + 3 - 2 * 9
// -3 * x ^ 2
// (1 + y) * 4.1
// sin PI
// fn(3, x + 1, sin PI / 2)
