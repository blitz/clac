//! # Calculator Types
//!
//! This module contains all types that are used to describe
//! calculator operations.

use std::convert::From;

/// A value on the stack of the calculator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
}

impl Value {
    pub fn is_float(&self) -> bool {
        if let Value::Float(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_integer(&self) -> bool {
        if let Value::Integer(_) = self {
            true
        } else {
            false
        }
    }
}

impl From<Value> for i64 {
    fn from(v: Value) -> Self {
        match v {
            Value::Integer(i) => i,
            Value::Float(f) => f as i64,
        }
    }
}

impl From<Value> for f64 {
    fn from(v: Value) -> Self {
        match v {
            // XXX This conversion will produce garbage if i is out of
            // range for the float.
            Value::Integer(i) => i as f64,

            Value::Float(f) => f,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
        }
    }
}

/// An operation that can be run on a calculator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Push(Value),
    Add,
    Subtract,
    Multiply,
    Divide,
    BitAnd,
    BitOr,
    BitXor,
    LeftShift,
    RightShift,
}
