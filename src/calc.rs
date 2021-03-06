//! # A Reverse Polish Calculator Engine
//!
//! This module contains the heart of the calculator. It is a
//! stack-based calculator works somewhat like a classic [HP
//! 48](https://en.wikipedia.org/wiki/HP_48_series) calculator.

use std::convert::TryInto;

use crate::types::{Operation, Radix, Value};

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

#[derive(Debug, Clone)]
#[must_use]
pub struct Calculator {
    value_stack: Vec<Value>,
    output_radix: Radix,
}

/// A generic type for all kinds of calculator operation
/// implementations.
trait OpImpl {
    fn execute(&self, calc: &mut Calculator) -> Result<(), CalculatorError>;
}

/// Swap two values on the stack.
#[derive(Default)]
struct SwapImpl {}

impl OpImpl for SwapImpl {
    fn execute(&self, calc: &mut Calculator) -> Result<(), CalculatorError> {
        let a = calc.pop_mut()?;
        let b = calc.pop_mut()?;

        calc.push_mut(a);
        calc.push_mut(b);

        Ok(())
    }
}

/// Set the output base of the calculator.
struct SetRadixImpl {
    radix: Radix,
}

impl From<Radix> for SetRadixImpl {
    fn from(radix: Radix) -> Self {
        SetRadixImpl { radix }
    }
}

impl OpImpl for SetRadixImpl {
    fn execute(&self, calc: &mut Calculator) -> Result<(), CalculatorError> {
        calc.set_radix(self.radix);

        Ok(())
    }
}

/// Push a value onto the stack
struct PushImpl {
    value: Value,
}

impl From<Value> for PushImpl {
    fn from(value: Value) -> Self {
        PushImpl { value }
    }
}

impl OpImpl for PushImpl {
    fn execute(&self, calc: &mut Calculator) -> Result<(), CalculatorError> {
        calc.push_mut(self.value);

        Ok(())
    }
}

/// Bit flip a value. This autoconverts to integer.
///
/// TODO: It would be very nice to have something like
/// TwoParamOpImpl, but that causes the generic trait
/// implementations to conflict.
#[derive(Default)]
struct BitNotImpl {}

impl OpImpl for BitNotImpl {
    fn execute(&self, calc: &mut Calculator) -> Result<(), CalculatorError> {
        let a = i64::from(calc.pop_mut()?);

        calc.push_mut(Value::Integer(!a));
        Ok(())
    }
}

/// Any two parameter operation that produces a single output.
trait TwoParamOpImpl {
    fn compute(&self, a: Value, b: Value) -> Result<Value, CalculatorError>;
}

impl<T: TwoParamOpImpl> OpImpl for T {
    fn execute(&self, calc: &mut Calculator) -> Result<(), CalculatorError> {
        let b = calc.pop_mut()?;
        let a = calc.pop_mut()?;

        Ok(calc.push_mut(self.compute(a, b)?))
    }
}

/// A two parameter operation that promotes both its arguments to
/// float, if any one of it is.
struct TwoParamFloatPromoOpImpl {
    int_op: Box<dyn Fn(i64, i64) -> Result<Value, CalculatorError>>,
    float_op: Box<dyn Fn(f64, f64) -> Result<Value, CalculatorError>>,
}

impl TwoParamFloatPromoOpImpl {
    fn new(
        int_op: impl Fn(i64, i64) -> Result<Value, CalculatorError> + 'static,
        float_op: impl Fn(f64, f64) -> Result<Value, CalculatorError> + 'static,
    ) -> Self {
        TwoParamFloatPromoOpImpl {
            int_op: Box::new(int_op),
            float_op: Box::new(float_op),
        }
    }
}

impl TwoParamOpImpl for TwoParamFloatPromoOpImpl {
    fn compute(&self, a: Value, b: Value) -> Result<Value, CalculatorError> {
        if a.is_float() || b.is_float() {
            (self.float_op)(a.into(), b.into())
        } else {
            (self.int_op)(a.into(), b.into())
        }
    }
}

/// A two parameter operation that promotes both its arguments to
/// integers all the time.
struct TwoParamIntPromoOpImpl {
    int_op: Box<dyn Fn(i64, i64) -> Result<Value, CalculatorError>>,
}

impl TwoParamIntPromoOpImpl {
    fn new(int_op: impl Fn(i64, i64) -> Result<Value, CalculatorError> + 'static) -> Self {
        TwoParamIntPromoOpImpl {
            int_op: Box::new(int_op),
        }
    }
}

impl TwoParamOpImpl for TwoParamIntPromoOpImpl {
    fn compute(&self, a: Value, b: Value) -> Result<Value, CalculatorError> {
        (self.int_op)(i64::from(a), i64::from(b))
    }
}

impl From<Operation> for Box<dyn OpImpl> {
    fn from(op: Operation) -> Self {
        match op {
            Operation::Add => Box::new(TwoParamFloatPromoOpImpl::new(
                |a, b| -> Result<Value, CalculatorError> {
                    Ok(Value::Integer(
                        a.checked_add(b).ok_or(CalculatorError::InvalidOperation)?,
                    ))
                },
                |a, b| -> Result<Value, CalculatorError> { Ok(Value::Float(a + b)) },
            )),

            Operation::BitAnd => Box::new(TwoParamIntPromoOpImpl::new(
                |a, b| -> Result<Value, CalculatorError> { Ok(Value::Integer(a & b)) },
            )),

            Operation::BitNot => Box::new(BitNotImpl::default()),

            Operation::BitOr => Box::new(TwoParamIntPromoOpImpl::new(
                |a, b| -> Result<Value, CalculatorError> { Ok(Value::Integer(a | b)) },
            )),

            Operation::BitXor => Box::new(TwoParamIntPromoOpImpl::new(
                |a, b| -> Result<Value, CalculatorError> { Ok(Value::Integer(a ^ b)) },
            )),

            Operation::Divide => Box::new(TwoParamFloatPromoOpImpl::new(
                |a, b| -> Result<Value, CalculatorError> {
                    Ok(Value::Integer(
                        a.checked_div(b).ok_or(CalculatorError::InvalidOperation)?,
                    ))
                },
                |a, b| -> Result<Value, CalculatorError> { Ok(Value::Float(a / b)) },
            )),

            Operation::LeftShift => Box::new(TwoParamIntPromoOpImpl::new(
                |a, b| -> Result<Value, CalculatorError> {
                    Ok(Value::Integer(
                        a.checked_shl(
                            b.try_into()
                                .map_err(|_| CalculatorError::InvalidOperation)?,
                        )
                        .ok_or(CalculatorError::InvalidOperation)?,
                    ))
                },
            )),

            Operation::Multiply => Box::new(TwoParamFloatPromoOpImpl::new(
                |a, b| -> Result<Value, CalculatorError> {
                    Ok(Value::Integer(
                        a.checked_mul(b).ok_or(CalculatorError::InvalidOperation)?,
                    ))
                },
                |a, b| -> Result<Value, CalculatorError> { Ok(Value::Float(a * b)) },
            )),

            Operation::SetRadix(r) => Box::new(SetRadixImpl::from(r)),

            Operation::Swap => Box::new(SwapImpl::default()),

            Operation::Subtract => Box::new(TwoParamFloatPromoOpImpl::new(
                |a, b| -> Result<Value, CalculatorError> {
                    Ok(Value::Integer(
                        a.checked_sub(b).ok_or(CalculatorError::InvalidOperation)?,
                    ))
                },
                |a, b| -> Result<Value, CalculatorError> { Ok(Value::Float(a - b)) },
            )),

            Operation::RightShift => Box::new(TwoParamIntPromoOpImpl::new(
                |a, b| -> Result<Value, CalculatorError> {
                    Ok(Value::Integer(
                        a.checked_shr(
                            b.try_into()
                                .map_err(|_| CalculatorError::InvalidOperation)?,
                        )
                        .ok_or(CalculatorError::InvalidOperation)?,
                    ))
                },
            )),

            Operation::Push(v) => Box::new(PushImpl::from(v)),
        }
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

impl Calculator {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            value_stack: vec![],
            output_radix: Radix::Dec,
        }
    }

    pub fn pop_mut(&mut self) -> Result<Value, CalculatorError> {
        self.value_stack
            .pop()
            .ok_or(CalculatorError::StackUnderflow)
    }

    pub fn push_mut(&mut self, v: Value) {
        self.value_stack.push(v)
    }

    #[allow(dead_code)]
    pub fn push(&self, v: Value) -> Calculator {
        let mut new_calc = self.clone();

        new_calc.push_mut(v);
        new_calc
    }

    /// Apply a single operation on the calculator.
    pub fn apply_mut(&mut self, op: Operation) -> Result<(), CalculatorError> {
        Box::<dyn OpImpl>::from(op).execute(self)
    }

    /// A side-effect free version of [apply_mut] that returns a new
    /// calculator with the result.
    #[allow(dead_code)]
    pub fn apply(&self, op: Operation) -> Result<Self, CalculatorError> {
        let mut new_calc = self.clone();

        new_calc.apply_mut(op)?;
        Ok(new_calc)
    }

    pub fn set_radix(&mut self, radix: Radix) {
        self.output_radix = radix;
    }

    #[allow(dead_code)]
    pub fn stack(&self) -> &[Value] {
        &self.value_stack
    }
}

impl std::fmt::Display for Calculator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let tokens: Vec<String> = self
            .value_stack
            .iter()
            .map(|v| match v {
                Value::Integer(i) => match self.output_radix {
                    Radix::Dec => format!("{}", i),
                    Radix::Hex => format!("{:#x}", i),
                    Radix::Bin => format!("{:#b}", i),
                },
                Value::Float(fl) => format!("{}", fl),
            })
            .collect();

        write!(f, "{}", tokens.join(" "))
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
