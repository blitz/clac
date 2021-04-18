use anyhow::Result;
use num::Integer;
use std::error::Error;
use std::fmt::Display;

use crate::rpcalc::Operation;

#[derive(Debug, Clone)]
pub struct ParseError {
    token: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "Failed to parse \"{}\" as number or operation.",
            self.token
        )
    }
}

impl Error for ParseError {}

fn parse_one<T: Integer + std::str::FromStr>(token: &str) -> Result<Operation<T>> {
    Ok(match token {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        maybe_int => Operation::Value(maybe_int.parse::<T>().map_err(|_| ParseError {
            token: maybe_int.to_string(),
        })?),
    })
}

pub fn parse<T: Integer + std::str::FromStr>(line: &str) -> Result<Vec<Operation<T>>> {
    line.split_whitespace().map(|t| parse_one(t)).collect()
}
