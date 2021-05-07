//! # A Reverse Polish Calculator Engine
//!
//! This module contains the heart of the calculator. It is a
//! stack-based calculator works somewhat like a classic [HP
//! 48](https://en.wikipedia.org/wiki/HP_48_series) calculator.

use crate::types::{Operation, Value};

/// All errors that happen during calculation are represented by this
/// type.
#[derive(Debug, Clone, Copy)]
pub enum CalculatorError {
    StackUnderflow,
    InvalidOperation,
}

impl std::fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CalculatorError::StackUnderflow => write!(f, "Stack Underflow"),
            CalculatorError::InvalidOperation => {
                write!(f, "Invalid operation (overflow, divide by zero, ...)")
            }
        }
    }
}

impl std::error::Error for CalculatorError {}

#[derive(Debug, Clone, PartialEq, Default)]
#[must_use]
pub struct Calculator {
    value_stack: Vec<Value>,
}

fn any_is_float(a: Value, b: Value) -> bool {
    a.is_float() || b.is_float()
}

fn add(a: Value, b: Value) -> Result<Value, CalculatorError> {
    if any_is_float(a, b) {
        Ok(Value::Float(f64::from(a) + f64::from(b)))
    } else {
        Ok(Value::Integer(
            i64::from(a)
                .checked_add(b.into())
                .ok_or(CalculatorError::InvalidOperation)?,
        ))
    }
}

fn divide(a: Value, b: Value) -> Result<Value, CalculatorError> {
    if any_is_float(a, b) {
        Ok(Value::Float(f64::from(a) / f64::from(b)))
    } else {
        Ok(Value::Integer(
            i64::from(a)
                .checked_div(b.into())
                .ok_or(CalculatorError::InvalidOperation)?,
        ))
    }
}

impl Calculator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pop_mut(&mut self) -> Result<Value, CalculatorError> {
        self.value_stack
            .pop()
            .ok_or(CalculatorError::StackUnderflow)
    }

    pub fn push_mut(&mut self, v: Value) {
        self.value_stack.push(v)
    }

    pub fn push(&self, v: Value) -> Calculator {
        let mut new_calc = self.clone();

        new_calc.push_mut(v);
        new_calc
    }

    /// Apply a single operation on the calculator.
    pub fn apply_mut(&mut self, op: Operation) -> Result<(), CalculatorError> {
        match op {
            Operation::Push(v) => self.push_mut(v),
            Operation::Add => {
                let b = self.pop_mut()?;
                let a = self.pop_mut()?;

                self.push_mut(add(a, b)?);
            }
            Operation::Divide => {
                let b = self.pop_mut()?;
                let a = self.pop_mut()?;

                self.push_mut(divide(a, b)?);
            }
            _ => unimplemented!("{:?}", op),
        }

        Ok(())
    }

    /// A side-effect free version of [apply_mut] that returns a new
    /// calculator with the result.
    pub fn apply(&self, op: Operation) -> Result<Self, CalculatorError> {
        let mut new_calc = self.clone();

        new_calc.apply_mut(op)?;
        Ok(new_calc)
    }

    pub fn stack(&self) -> &[Value] {
        &self.value_stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() -> Result<(), CalculatorError> {
        assert!(Calculator::new().stack().is_empty());

        assert_eq!(
            Calculator::new()
                .apply(Operation::Push(Value::Integer(1)))?
                .stack(),
            &[Value::Integer(1)]
        );

        Ok(())
    }

    #[test]
    fn test_integer_2op() -> Result<(), CalculatorError> {
        // An operation where parameter order does not matter.
        assert_eq!(
            Calculator::new()
                .push(Value::Integer(1))
                .push(Value::Integer(2))
                .apply(Operation::Add)?
                .stack(),
            &[Value::Integer(3)]
        );

        // An operation where parameter order does matter.
        assert_eq!(
            Calculator::new()
                .push(Value::Integer(6))
                .push(Value::Integer(2))
                .apply(Operation::Divide)?
                .stack(),
            &[Value::Integer(3)]
        );

        Ok(())
    }

    #[test]
    fn test_float_promo() -> Result<(), CalculatorError> {
        // An addition with one float parameter becomes a float
        // addtion.
        assert_eq!(
            Calculator::new()
                .push(Value::Integer(1))
                .push(Value::Float(2.0))
                .apply(Operation::Add)?
                .stack(),
            &[Value::Float(3.0)]
        );

        assert_eq!(
            Calculator::new()
                .push(Value::Float(2.0))
                .push(Value::Integer(1))
                .apply(Operation::Add)?
                .stack(),
            &[Value::Float(3.0)]
        );

        assert_eq!(
            Calculator::new()
                .push(Value::Float(1.0))
                .push(Value::Float(2.0))
                .apply(Operation::Add)?
                .stack(),
            &[Value::Float(3.0)]
        );

        Ok(())
    }
}
