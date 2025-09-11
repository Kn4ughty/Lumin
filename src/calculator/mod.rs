use anyhow;
use anyhow::bail;
use iced::{Element, futures::future::OkInto, widget};

// This is my fav so far
// https://docs.rs/crate/calculator-lib/0.1.1/source/src/lib.rs

// https://itnext.io/writing-a-mathematical-expression-parser-35b0b78f869e
// https://compilers.iecc.com/crenshaw/
// https://craftinginterpreters.com/contents.html

// use crate::module::Module;
//
pub struct Calc {}
//
// impl Module for Calc {
//     fn update(&mut self, input: &str) {}
//     fn view(&self) -> Element<'_, String> {}
//     fn run(&self) {
//         // Copy to clipboard?
//     }
// }

const BASE: u32 = 10;

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

impl Expr {
    fn get_number(&self) -> anyhow::Result<f64> {
        if let Self::Number(num) = self {
            return Ok(*num);
        } else {
            bail!("Expression was not number")
        }
    }
}

impl Calc {
    fn tokenize(source: &str) -> anyhow::Result<Vec<Expr>> {
        let mut out = Vec::new();
        let mut chars = source.chars().peekable();

        let mut current: String = "".to_string();

        // TODO. Convert to use enumerate
        while let Some(c) = chars.next() {
            if c.is_digit(BASE) || c == '.' {
                current.push(c);
                match chars.peek() {
                    Some('.') | Some('0'..='9') => current.push(chars.next().unwrap()),
                    // No number next
                    _ => {
                        out.push(Expr::Number(current.parse().unwrap())); //UNSAFE
                        current.clear();
                    }
                }
                continue;
            }

            if current.len() != 0 {
                out.push(Expr::Number(current.parse().unwrap()));
                current.clear();
                continue;
            }

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
                        1
                    ))
                }
            });
        }

        Ok(out)
    }

    fn evaluate_brackets(mut input: Vec<Expr>) -> anyhow::Result<Vec<Expr>> {
        println!("Running evaluate_brackets with input vec: {input:?}");
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

    // fn calc(mut input: Vec<Expr>) -> anyhow::Result<f64> {
    //     let iter_idx = 0;
    //     let last_expr: Option<Expr> = None;
    //
    //     while input.len() < iter_idx {
    //         let expr = input[iter_idx].clone();
    //
    //         match expr {
    //             Expr::Bracket(inner) => {
    //                 // handle inner. (recurse)
    //             }
    //             Expr::OpenParen => unreachable!(),
    //             Expr::CloseParen => unreachable!(),
    //             _ => { // Must be operator
    //                 let lhs = match input.clone().get(iter_idx - 1).ok_or(
    //                     CalcError::from_expr_list(
    //                         "Expression not found".to_string(),
    //                         input.clone(),
    //                         iter_idx)
    //                 )? {
    //                     Expr::Bracket(inner) => inner.clone(),
    //                     Expr::Number(inner) => inner.clone(),
    //                     _ => CalcError
    //                 };
    //             }
    //         }
    //     }
    //
    //     Ok(input[0].get_number()?)
    // }
}

#[test]
fn can_convert_source_to_tokenvec() {
    assert_eq!(
        Calc::tokenize("1 + 1-1*2.7").unwrap(),
        vec![
            Expr::Number(1.0),
            Expr::Plus,
            Expr::Number(1.0),
            Expr::Minus,
            Expr::Number(1.0),
            Expr::Multiply,
            Expr::Number(2.7),
        ]
    )
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

// fn evaluate(mut input: Vec<Expr>) -> anyhow::Result<Vec<Expr>> {
//     // BODMAS
//     // Walk with stack of brackets. Evaluate first closed item found
//     let mut stack = Vec::new();
//     let mut values = Vec::new();
//
//     for (i, tok) in input.iter().enumerate() {
//     }
//
//     Ok(input)
// }

#[derive(Debug)]
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

    // fn from_expr_list(message: String, equation: Vec<Expr>, error_idx: usize) -> Self {
    //
    // }

}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("hello")
    }
}

// TEST THESE
// 4 + 3 - 2 * 9
// -3 * x ^ 2
// (1 + y) * 4.1
// sin PI
// fn(3, x + 1, sin PI / 2)
