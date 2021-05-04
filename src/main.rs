mod rpcalc;
mod traits;

use anyhow::{Context, Result};
use std::io::{self, BufRead, Write};
use std::iter::Iterator;
use std::str::FromStr;

use crate::rpcalc::{Calculator, Operation};
use crate::traits::Number;

fn prompt<T: Number>(calc: &Calculator<T>) -> String {
    let tokens: Vec<String> = calc.stack().iter().map(|v| v.to_string()).collect();

    tokens.join(" ")
}

/// Takes a line of user input and turns it into operations that can
/// run the calculator.
fn parse_line<T: Number>(line: &str) -> Result<Vec<Operation<T>>> {
    line.split_whitespace()
        .map(|t| Operation::<T>::from_str(t).context("Failed to parse token"))
        .collect()
}

fn parse_and_do<T: Number>(calc: &Calculator<T>, line: &str) -> Result<Calculator<T>> {
    let mut new_calc = calc.clone();

    for op in parse_line(&line)? {
        new_calc.do_operation(op)?
    }

    Ok(new_calc)
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut calc = Calculator::<i64>::default();
    let mut lines = stdin.lock().lines();

    loop {
        print!("{} | ", prompt(&calc));
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
