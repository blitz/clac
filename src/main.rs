mod calc;
mod parser;
mod types;

use anyhow::Result;
use std::io::{self, BufRead, Write};
use std::iter::Iterator;

use crate::calc::Calculator;
use crate::parser::parse;

fn parse_and_do(calc: &Calculator, line: &str) -> Result<Calculator> {
    let mut new_calc = calc.clone();

    for op in parse(&line)? {
        new_calc.apply_mut(op)?
    }

    Ok(new_calc)
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut calc = Calculator::default();
    let mut lines = stdin.lock().lines();

    loop {
        print!("{} | ", calc);
        io::stdout().flush()?;

        match lines.next() {
            Some(line) => match parse_and_do(&calc, &line?) {
                Ok(new_calc) => calc = new_calc,
                Err(e) => println!("Error: {}", e),
            },
            None => break,
        }
    }

    Ok(())
}
