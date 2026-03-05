//! Arithmetic, comparison, and bitwise operators for the interpreter.
//
//! All op_* methods are pure computations on Value types.

use super::interpreter::{Interpreter, InterpreterError, InterpreterResult};
use crate::value::Value;

impl Interpreter {
    // Arithmetic operations

    pub(crate) fn op_neg(&self, val: Value) -> InterpreterResult<Value> {
        if let Some(n) = val.to_i32() {
            if n == i32::MIN {
                // Overflow: would need f64
                return Err(InterpreterError::InternalError(
                    "integer overflow".to_string(),
                ));
            }
            Ok(Value::int(-n))
        } else {
            Err(InterpreterError::TypeError(
                "cannot negate non-number".to_string(),
            ))
        }
    }

    pub(crate) fn op_add(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        match (a.to_i32(), b.to_i32()) {
            (Some(va), Some(vb)) => Ok(Value::int(va.wrapping_add(vb))),
            _ => Err(InterpreterError::TypeError(
                "cannot add non-numbers".to_string(),
            )),
        }
    }

    pub(crate) fn op_sub(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        match (a.to_i32(), b.to_i32()) {
            (Some(va), Some(vb)) => Ok(Value::int(va.wrapping_sub(vb))),
            _ => Err(InterpreterError::TypeError(
                "cannot subtract non-numbers".to_string(),
            )),
        }
    }

    pub(crate) fn op_mul(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        match (a.to_i32(), b.to_i32()) {
            (Some(va), Some(vb)) => Ok(Value::int(va.wrapping_mul(vb))),
            _ => Err(InterpreterError::TypeError(
                "cannot multiply non-numbers".to_string(),
            )),
        }
    }

    pub(crate) fn op_div(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        match (a.to_i32(), b.to_i32()) {
            (Some(va), Some(vb)) => {
                if vb == 0 {
                    Err(InterpreterError::DivisionByZero)
                } else if let Some(result) = va.checked_div(vb) {
                    Ok(Value::int(result))
                } else {
                    Err(InterpreterError::InternalError(
                        "integer overflow".to_string(),
                    ))
                }
            }
            _ => Err(InterpreterError::TypeError(
                "cannot divide non-numbers".to_string(),
            )),
        }
    }

    pub(crate) fn op_mod(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        match (a.to_i32(), b.to_i32()) {
            (Some(va), Some(vb)) => {
                if vb == 0 {
                    Err(InterpreterError::DivisionByZero)
                } else if let Some(result) = va.checked_rem(vb) {
                    Ok(Value::int(result))
                } else {
                    Err(InterpreterError::InternalError(
                        "integer overflow".to_string(),
                    ))
                }
            }
            _ => Err(InterpreterError::TypeError(
                "cannot modulo non-numbers".to_string(),
            )),
        }
    }

    // Comparison operations

    pub(crate) fn op_lt(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        match (a.to_i32(), b.to_i32()) {
            (Some(va), Some(vb)) => Ok(Value::bool(va < vb)),
            _ => Err(InterpreterError::TypeError(
                "cannot compare non-numbers".to_string(),
            )),
        }
    }

    pub(crate) fn op_lte(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        match (a.to_i32(), b.to_i32()) {
            (Some(va), Some(vb)) => Ok(Value::bool(va <= vb)),
            _ => Err(InterpreterError::TypeError(
                "cannot compare non-numbers".to_string(),
            )),
        }
    }

    pub(crate) fn op_gt(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        match (a.to_i32(), b.to_i32()) {
            (Some(va), Some(vb)) => Ok(Value::bool(va > vb)),
            _ => Err(InterpreterError::TypeError(
                "cannot compare non-numbers".to_string(),
            )),
        }
    }

    pub(crate) fn op_gte(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        match (a.to_i32(), b.to_i32()) {
            (Some(va), Some(vb)) => Ok(Value::bool(va >= vb)),
            _ => Err(InterpreterError::TypeError(
                "cannot compare non-numbers".to_string(),
            )),
        }
    }

    pub(crate) fn op_eq(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        // Simple equality for now (strict equality)
        Ok(Value::bool(a == b))
    }

    pub(crate) fn op_neq(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        Ok(Value::bool(a != b))
    }

    // Bitwise operations

    pub(crate) fn op_bitwise_not(&self, val: Value) -> InterpreterResult<Value> {
        if let Some(n) = val.to_i32() {
            Ok(Value::int(!n))
        } else {
            Err(InterpreterError::TypeError(
                "cannot apply bitwise NOT to non-number".to_string(),
            ))
        }
    }

    pub(crate) fn op_bitwise_and(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        match (a.to_i32(), b.to_i32()) {
            (Some(va), Some(vb)) => Ok(Value::int(va & vb)),
            _ => Err(InterpreterError::TypeError(
                "cannot apply bitwise AND to non-numbers".to_string(),
            )),
        }
    }

    pub(crate) fn op_bitwise_or(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        match (a.to_i32(), b.to_i32()) {
            (Some(va), Some(vb)) => Ok(Value::int(va | vb)),
            _ => Err(InterpreterError::TypeError(
                "cannot apply bitwise OR to non-numbers".to_string(),
            )),
        }
    }

    pub(crate) fn op_bitwise_xor(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        match (a.to_i32(), b.to_i32()) {
            (Some(va), Some(vb)) => Ok(Value::int(va ^ vb)),
            _ => Err(InterpreterError::TypeError(
                "cannot apply bitwise XOR to non-numbers".to_string(),
            )),
        }
    }

    pub(crate) fn op_shl(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        match (a.to_i32(), b.to_i32()) {
            (Some(va), Some(vb)) => {
                let shift = (vb & 0x1f) as u32;
                Ok(Value::int(va << shift))
            }
            _ => Err(InterpreterError::TypeError(
                "cannot apply left shift to non-numbers".to_string(),
            )),
        }
    }

    pub(crate) fn op_sar(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        match (a.to_i32(), b.to_i32()) {
            (Some(va), Some(vb)) => {
                let shift = (vb & 0x1f) as u32;
                Ok(Value::int(va >> shift))
            }
            _ => Err(InterpreterError::TypeError(
                "cannot apply arithmetic right shift to non-numbers".to_string(),
            )),
        }
    }

    pub(crate) fn op_shr(&self, a: Value, b: Value) -> InterpreterResult<Value> {
        match (a.to_i32(), b.to_i32()) {
            (Some(va), Some(vb)) => {
                let shift = (vb & 0x1f) as u32;
                let result = (va as u32) >> shift;
                Ok(Value::int(result as i32))
            }
            _ => Err(InterpreterError::TypeError(
                "cannot apply logical right shift to non-numbers".to_string(),
            )),
        }
    }
}
