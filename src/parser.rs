//! # Input Parsing
//!
//! In this module, we parse input into calculator operations.

use lazy_static::lazy_static;
use regex::Regex;

use std::str::FromStr;

use crate::types::{Operation, Value};

/// All parsing errors are represented by this type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// The token that could not be parsed.
    offending_token: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Failed to parse token: {}", self.offending_token)
    }
}

impl std::error::Error for ParseError {}

fn parse_value(token: &str) -> Result<Value, ParseError> {
    let parse_error = || ParseError {
        offending_token: token.to_string(),
    };

    lazy_static! {
        static ref INTEGER_RE: Regex = Regex::new("^[0-9]+$").unwrap();
        static ref FLOAT_RE: Regex = Regex::new("^[0-9]+\\.[0-9]+$").unwrap();
    }

    if INTEGER_RE.is_match(token) {
        Ok(Value::Integer(
            i64::from_str(token).map_err(|_| parse_error())?,
        ))
    } else if FLOAT_RE.is_match(token) {
        Ok(Value::Float(
            f64::from_str(token).map_err(|_| parse_error())?,
        ))
    } else {
        Err(parse_error())
    }
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(token: &str) -> Result<Self, ParseError> {
        match token {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "*" => Ok(Operation::Multiply),
            "/" => Ok(Operation::Divide),
            "&" => Ok(Operation::BitAnd),
            "|" => Ok(Operation::BitOr),
            "^" => Ok(Operation::BitXor),
            "<<" => Ok(Operation::LeftShift),
            ">>" => Ok(Operation::RightShift),
            _ => Ok(Operation::Push(parse_value(token)?)),
        }
    }
}

/// Parse a single line of input into a sequence of calculator
/// operations.
pub fn parse(input: &str) -> Result<Vec<Operation>, ParseError> {
    input
        .split_whitespace()
        .map(|t| Operation::from_str(t))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        assert_eq!(Operation::from_str("+"), Ok(Operation::Add));
        assert_eq!(Operation::from_str("<<"), Ok(Operation::LeftShift));
        assert_eq!(
            Operation::from_str("16"),
            Ok(Operation::Push(Value::Integer(16)))
        );
        assert_eq!(
            Operation::from_str("16.0"),
            Ok(Operation::Push(Value::Float(16.0)))
        );
        assert_eq!(
            Operation::from_str("13x213!"),
            Err(ParseError {
                offending_token: "13x213!".to_string()
            })
        );
    }

    #[test]
    fn test_multiple() {
        assert_eq!(parse(""), Ok(vec![]));
        assert_eq!(
            parse("1 2 +"),
            Ok(vec![
                Operation::Push(Value::Integer(1)),
                Operation::Push(Value::Integer(2)),
                Operation::Add
            ])
        );

        assert_eq!(
            parse("1 2 xxzz! 3"),
            Err(ParseError {
                offending_token: "xxzz!".to_string()
            })
        )
    }
}
