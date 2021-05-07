//! # Calculator Types
//!
//! This module contains all types that are used to describe
//! calculator operations.

/// A value on the stack of the calculator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
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
