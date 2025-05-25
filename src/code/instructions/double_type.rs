use std::any::type_name_of_val;
use libgm::gm::{GMDataType, GMValue};
use crate::code::run::GMStack;
use std::convert::TryInto;

pub fn conv(stack: &mut GMStack, target_data_type: GMDataType) -> Result<(), String> {
    let old: GMValue = stack.pop()?;
    let new: GMValue = match target_data_type {
        GMDataType::Double => GMValue::Double(match old {
            GMValue::Double(val) => val,
            GMValue::Float(val) => f64::from(val),
            GMValue::Int16(val) => f64::from(val),
            GMValue::Int32(val) => f64::from(val),
            GMValue::Int64(val) => f64::from(val as i32),
            GMValue::Boolean(val) => f64::from(val),
            other => return Err(format!("Invalid source conversion Data Type {other:?} while converting to Double"))
        }),
        GMDataType::Float => GMValue::Float(match old {
            GMValue::Double(val) => val as f32,
            GMValue::Float(val) => val,
            GMValue::Int16(val) => f32::from(val),
            GMValue::Int32(val) => val as f32,
            GMValue::Int64(val) => val as f32,
            GMValue::Boolean(val) => f32::from(val),
            other => return Err(format!("Invalid source conversion Data Type {other:?} while converting to Float"))
        }),
        GMDataType::Int16 => GMValue::Int16(match old {
            GMValue::Double(val) => val as i16,
            GMValue::Float(val) => val as i16,
            GMValue::Int16(val) => val,
            GMValue::Int32(val) => val as i16,
            GMValue::Int64(val) => val as i16,
            GMValue::Boolean(val) => i16::from(val),
            other => return Err(format!("Invalid source conversion Data Type {other:?} while converting to Int16"))
        }),
        GMDataType::Int32 => GMValue::Int32(match old {
            GMValue::Double(val) => val as i32,
            GMValue::Float(val) => val as i32,
            GMValue::Int16(val) => i32::from(val),
            GMValue::Int32(val) => val,
            GMValue::Int64(val) => val as i32,
            GMValue::Boolean(val) => i32::from(val),
            other => return Err(format!("Invalid source conversion Data Type {other:?} while converting to Int32"))
        }),
        GMDataType::Int64 => GMValue::Int64(match old {
            GMValue::Double(val) => val as i64,
            GMValue::Float(val) => val as i64,
            GMValue::Int16(val) => i64::from(val),
            GMValue::Int32(val) => i64::from(val),
            GMValue::Int64(val) => val,
            GMValue::Boolean(val) => i64::from(val),
            other => return Err(format!("Invalid source conversion Data Type {other:?} while converting to Int64"))
        }),
        GMDataType::Boolean => GMValue::Boolean(match old {
            GMValue::Double(val) => val == 1.0,     // !!!! this is probably bad
            GMValue::Float(val) => val == 1.0,      // !!!!
            GMValue::Int16(val) => val == 1,
            GMValue::Int32(val) => val == 1,
            GMValue::Int64(val) => val == 1,
            GMValue::Boolean(val) => val,
            other => return Err(format!("Invalid source conversion Data Type {other:?} while converting to Boolean"))
        }),
        other => return Err(format!("Invalid target conversion Data Type {other:?}"))
    };
    stack.push(new);
    Ok(())
}

pub fn mul(stack: &mut GMStack) -> Result<(), String> {
    let lhs: GMValue = stack.pop()?;
    let rhs: GMValue = stack.pop()?;
    let result: GMValue = match (lhs, rhs) {
        (GMValue::Double(a), GMValue::Double(b)) => GMValue::Double(a * b),
        (GMValue::Float(a), GMValue::Float(b)) => GMValue::Float(a * b),
        (GMValue::Int16(a), GMValue::Int16(b)) => GMValue::Int16(a * b),
        (GMValue::Int32(a), GMValue::Int32(b)) => GMValue::Int32(a * b),
        (GMValue::Int64(a), GMValue::Int64(b)) => GMValue::Int64(a * b),
        (a, b) => return Err(format!("Cannot multiply {:?} with {:?}", a, b)),
    };
    stack.push(result);
    Ok(())
}


pub fn div(stack: &mut GMStack) -> Result<(), String> {
    let lhs: GMValue = stack.pop()?;
    let rhs: GMValue = stack.pop()?;
    let result: GMValue = match (lhs, rhs) {
        (GMValue::Double(a), GMValue::Double(b)) => GMValue::Double(a / b),
        (GMValue::Float(a), GMValue::Float(b)) => GMValue::Float(a / b),
        (GMValue::Int16(a), GMValue::Int16(b)) => GMValue::Int16(safe_div(a, b)?),
        (GMValue::Int32(a), GMValue::Int32(b)) => GMValue::Int32(safe_div(a, b)?),
        (GMValue::Int64(a), GMValue::Int64(b)) => GMValue::Int64(safe_div(a, b)?),
        (a, b) => return Err(format!("Cannot divide {:?} by {:?}", a, b)),
    };
    stack.push(result);
    Ok(())
}


pub fn rem(stack: &mut GMStack) -> Result<(), String> {
    let lhs: GMValue = stack.pop()?;
    let rhs: GMValue = stack.pop()?;
    let result: GMValue = match (lhs, rhs) {
        (GMValue::Double(a), GMValue::Double(b)) => GMValue::Double(a.rem_euclid(b)),
        (GMValue::Float(a), GMValue::Float(b)) => GMValue::Float(a.rem_euclid(b)),
        (GMValue::Int16(a), GMValue::Int16(b)) => GMValue::Int16(a.checked_rem(b).ok_or_else(|| format!("Failed to get remainder of {a} with divisor {b}: Division by zero"))?),
        (GMValue::Int32(a), GMValue::Int32(b)) => GMValue::Int32(a.checked_rem(b).ok_or_else(|| format!("Failed to get remainder of {a} with divisor {b}: Division by zero"))?),
        (GMValue::Int64(a), GMValue::Int64(b)) => GMValue::Int64(a.checked_rem(b).ok_or_else(|| format!("Failed to get remainder of {a} with divisor {b}: Division by zero"))?),
        (a, b) => return Err(format!("Cannot get remainder of type {:?} with divisor type {:?}", a, b)),
    };
    stack.push(result);
    Ok(())
}

pub fn mod_(stack: &mut GMStack) -> Result<(), String> {
    let lhs: GMValue = stack.pop()?;
    let rhs: GMValue = stack.pop()?;
    let result: GMValue = match (lhs, rhs) {
        (GMValue::Double(a), GMValue::Double(b)) => GMValue::Double(a % b),
        (GMValue::Float(a), GMValue::Float(b)) => GMValue::Float(a % b),
        (GMValue::Int16(a), GMValue::Int16(b)) => GMValue::Int16(safe_mod(a, b)?),
        (GMValue::Int32(a), GMValue::Int32(b)) => GMValue::Int32(safe_mod(a, b)?),
        (GMValue::Int64(a), GMValue::Int64(b)) => GMValue::Int64(safe_mod(a, b)?),
        (a, b) => return Err(format!("Cannot get modulus of type {:?} with divisor type {:?}", a, b)),
    };
    stack.push(result);
    Ok(())
}

pub fn add(stack: &mut GMStack) -> Result<(), String> {
    let lhs: GMValue = stack.pop()?;
    let rhs: GMValue = stack.pop()?;
    let result: GMValue = match (lhs, rhs) {
        (GMValue::Double(a), GMValue::Double(b)) => GMValue::Double(a + b),
        (GMValue::Float(a), GMValue::Float(b)) => GMValue::Float(a + b),
        (GMValue::Int16(a), GMValue::Int16(b)) => GMValue::Int16(a + b),
        (GMValue::Int32(a), GMValue::Int32(b)) => GMValue::Int32(a + b),
        (GMValue::Int64(a), GMValue::Int64(b)) => GMValue::Int64(a + b),
        (a, b) => return Err(format!("Cannot add {:?} to {:?}", a, b)),
    };
    stack.push(result);
    Ok(())
}

pub fn sub(stack: &mut GMStack) -> Result<(), String> {
    let lhs: GMValue = stack.pop()?;
    let rhs: GMValue = stack.pop()?;
    let result: GMValue = match (lhs, rhs) {
        (GMValue::Double(a), GMValue::Double(b)) => GMValue::Double(a - b),
        (GMValue::Float(a), GMValue::Float(b)) => GMValue::Float(a - b),
        (GMValue::Int16(a), GMValue::Int16(b)) => GMValue::Int16(a - b),
        (GMValue::Int32(a), GMValue::Int32(b)) => GMValue::Int32(a - b),
        (GMValue::Int64(a), GMValue::Int64(b)) => GMValue::Int64(a - b),
        (a, b) => return Err(format!("Cannot subtract {:?} from {:?}", a, b)),
    };
    stack.push(result);
    Ok(())
}

pub fn and(stack: &mut GMStack) -> Result<(), String> {
    let lhs: GMValue = stack.pop()?;
    let rhs: GMValue = stack.pop()?;
    let result: GMValue = match (lhs, rhs) {
        (GMValue::Int16(a), GMValue::Int16(b)) => GMValue::Int16(a & b),
        (GMValue::Int32(a), GMValue::Int32(b)) => GMValue::Int32(a & b),
        (GMValue::Int64(a), GMValue::Int64(b)) => GMValue::Int64(a & b),
        (GMValue::Boolean(a), GMValue::Boolean(b)) => GMValue::Boolean(a & b),
        (a, b) => return Err(format!("Cannot bitwise AND {:?} with {:?}", a, b)),
    };
    stack.push(result);
    Ok(())
}

pub fn or(stack: &mut GMStack) -> Result<(), String> {
    let lhs: GMValue = stack.pop()?;
    let rhs: GMValue = stack.pop()?;
    let result: GMValue = match (lhs, rhs) {
        (GMValue::Int16(a), GMValue::Int16(b)) => GMValue::Int16(a | b),
        (GMValue::Int32(a), GMValue::Int32(b)) => GMValue::Int32(a | b),
        (GMValue::Int64(a), GMValue::Int64(b)) => GMValue::Int64(a | b),
        (GMValue::Boolean(a), GMValue::Boolean(b)) => GMValue::Boolean(a | b),
        (a, b) => return Err(format!("Cannot bitwise OR {:?} with {:?}", a, b)),
    };
    stack.push(result);
    Ok(())
}

pub fn xor(stack: &mut GMStack) -> Result<(), String> {
    let lhs: GMValue = stack.pop()?;
    let rhs: GMValue = stack.pop()?;
    let result: GMValue = match (lhs, rhs) {
        (GMValue::Int16(a), GMValue::Int16(b)) => GMValue::Int16(a ^ b),
        (GMValue::Int32(a), GMValue::Int32(b)) => GMValue::Int32(a ^ b),
        (GMValue::Int64(a), GMValue::Int64(b)) => GMValue::Int64(a ^ b),
        (GMValue::Boolean(a), GMValue::Boolean(b)) => GMValue::Boolean(a ^ b),
        (a, b) => return Err(format!("Cannot bitwise XOR {:?} with {:?}", a, b)),
    };
    stack.push(result);
    Ok(())
}

pub fn shl(stack: &mut GMStack) -> Result<(), String> {
    let lhs: GMValue = stack.pop()?;
    let rhs: GMValue = stack.pop()?;
    let result: GMValue = match (lhs, rhs) {
        (GMValue::Int16(a), GMValue::Int16(b)) => GMValue::Int16(safe_shift_left(a, b as u32)?),
        (GMValue::Int16(a), GMValue::Int32(b)) => GMValue::Int16(safe_shift_left(a, b as u32)?),
        (GMValue::Int16(a), GMValue::Int64(b)) => GMValue::Int16(safe_shift_left(a, b as u32)?),
        (GMValue::Int32(a), GMValue::Int16(b)) => GMValue::Int32(safe_shift_left(a, b as u32)?),
        (GMValue::Int32(a), GMValue::Int32(b)) => GMValue::Int32(safe_shift_left(a, b as u32)?),
        (GMValue::Int32(a), GMValue::Int64(b)) => GMValue::Int32(safe_shift_left(a, b as u32)?),
        (GMValue::Int64(a), GMValue::Int16(b)) => GMValue::Int64(safe_shift_left(a, b as u32)?),
        (GMValue::Int64(a), GMValue::Int32(b)) => GMValue::Int64(safe_shift_left(a, b as u32)?),
        (GMValue::Int64(a), GMValue::Int64(b)) => GMValue::Int64(safe_shift_left(a, b as u32)?),
        (GMValue::Boolean(a), GMValue::Boolean(b)) => GMValue::Boolean(a ^ b),
        (a, b) => return Err(format!("Cannot left-bitshift type {:?} by type {:?}", a, b)),
    };
    stack.push(result);
    Ok(())
}

pub fn shr(stack: &mut GMStack) -> Result<(), String> {
    let lhs: GMValue = stack.pop()?;
    let rhs: GMValue = stack.pop()?;
    let result: GMValue = match (lhs, rhs) {
        (GMValue::Int16(a), GMValue::Int16(b)) => GMValue::Int16(safe_shift_right(a, b as u32)?),
        (GMValue::Int16(a), GMValue::Int32(b)) => GMValue::Int16(safe_shift_right(a, b as u32)?),
        (GMValue::Int16(a), GMValue::Int64(b)) => GMValue::Int16(safe_shift_right(a, b as u32)?),
        (GMValue::Int32(a), GMValue::Int16(b)) => GMValue::Int32(safe_shift_right(a, b as u32)?),
        (GMValue::Int32(a), GMValue::Int32(b)) => GMValue::Int32(safe_shift_right(a, b as u32)?),
        (GMValue::Int32(a), GMValue::Int64(b)) => GMValue::Int32(safe_shift_right(a, b as u32)?),
        (GMValue::Int64(a), GMValue::Int16(b)) => GMValue::Int64(safe_shift_right(a, b as u32)?),
        (GMValue::Int64(a), GMValue::Int32(b)) => GMValue::Int64(safe_shift_right(a, b as u32)?),
        (GMValue::Int64(a), GMValue::Int64(b)) => GMValue::Int64(safe_shift_right(a, b as u32)?),
        (GMValue::Boolean(a), GMValue::Boolean(b)) => GMValue::Boolean(a ^ b),
        (a, b) => return Err(format!("Cannot right-bitshift type {:?} by type {:?}", a, b)),
    };
    stack.push(result);
    Ok(())
}

fn safe_div<T: num_traits::ops::checked::CheckedDiv + std::fmt::Display>(lhs: T, rhs: T) -> Result<T, String> {
    lhs.checked_div(&rhs).ok_or_else(|| format!("Failed to divide {lhs} / {rhs}: Division by zero"))
}

fn safe_mod<T: num_traits::ops::checked::CheckedRem + std::fmt::Display>(lhs: T, rhs: T) -> Result<T, String> {
    lhs.checked_rem(&rhs).ok_or_else(|| format!("Failed to get the modulus {lhs} % {rhs}: Division by zero"))
}

fn safe_shift_left<T: num_traits::ops::checked::CheckedShl + std::fmt::Display>(lhs: T, rhs: u32) -> Result<T, String> {
    lhs.checked_shl(rhs).ok_or_else(|| format!("Failed to bitshift left {lhs} << {rhs}: Result overflowed"))
}

fn safe_shift_right<T: num_traits::ops::checked::CheckedShr + std::fmt::Display>(lhs: T, rhs: u32) -> Result<T, String> {
    lhs.checked_shr(rhs).ok_or_else(|| format!("Failed to bitshift right {lhs} >> {rhs}: Result overflowed"))
}
