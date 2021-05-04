use num::Integer;
use std::error::Error;
use std::fmt::Display;
use std::ops::FnOnce;
use std::str::FromStr;

/// All errors that happen during calculation are represented by this
/// type.
#[derive(Debug, Clone, Copy)]
pub enum CalculatorError {
    StackUnderflow,
    DivideByZero,
}

impl Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            CalculatorError::StackUnderflow => write!(f, "Stack Underflow"),
            CalculatorError::DivideByZero => write!(f, "Divide by zero"),
        }
    }
}

impl Error for CalculatorError {}

/// All parsing errors are represented by this type.
#[derive(Debug, Clone, Copy)]
pub struct ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Failed to parse token")
    }
}

impl Error for ParseError {}

#[derive(Default, Debug, Clone)]
pub struct Calculator<T: Integer + Clone> {
    stack: Vec<T>,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation<T: Integer> {
    Value(T),
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl<T: Integer + FromStr> FromStr for Operation<T> {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, ParseError> {
        match s {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "*" => Ok(Operation::Multiply),
            "/" => Ok(Operation::Divide),
            maybe_int => Ok(Operation::Value(
                maybe_int.parse::<T>().map_err(|_| ParseError {})?,
            )),
        }
    }
}

pub type CalculatorResult<T> = Result<T, CalculatorError>;

impl<T: Integer + Clone> Calculator<T> {
    fn pop(&mut self) -> CalculatorResult<T> {
        self.stack.pop().ok_or(CalculatorError::StackUnderflow)
    }

    fn push(&mut self, v: T) {
        self.stack.push(v);
    }

    fn do_2param_op<F>(&mut self, func: F) -> CalculatorResult<()>
    where
        F: FnOnce(T, T) -> CalculatorResult<T>,
    {
        let v1 = self.pop()?;
        let v2 = self.pop()?;
        let new_v = func(v2, v1)?;

        self.push(new_v);
        Ok(())
    }

    pub fn do_operation(&mut self, op: Operation<T>) -> CalculatorResult<()> {
        match op {
            Operation::Value(v) => self.push(v),
            Operation::Add => self.do_2param_op(|a, b| Ok(a + b))?,
            Operation::Subtract => self.do_2param_op(|a, b| Ok(a - b))?,
            Operation::Multiply => self.do_2param_op(|a, b| Ok(a * b))?,
            Operation::Divide => self.do_2param_op(|a, b| {
                if b.is_zero() {
                    Err(CalculatorError::DivideByZero)
                } else {
                    Ok(a / b)
                }
            })?,
        };

        Ok(())
    }

    pub fn stack(&self) -> Vec<T> {
        self.stack.clone()
    }
}
