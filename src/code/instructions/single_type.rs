use libgm::gm::GMValue;
use crate::code::run::GMStack;

pub fn neg(stack: &mut GMStack) -> Result<(), String> {
    let old: GMValue = stack.pop()?;
    let new: GMValue = match old {
        GMValue::Double(val) => GMValue::Double(-val),
        GMValue::Float(val) => GMValue::Float(-val),
        GMValue::Int16(val) => GMValue::Int16(-val),
        GMValue::Int32(val) => GMValue::Int32(-val),
        GMValue::Int64(val) => GMValue::Int64(-val),
        GMValue::Boolean(_) => return Err("Cannot int negate boolean value".to_string()),
        GMValue::String(_) => return Err("Cannot int negate string value".to_string()),
        GMValue::Variable(_) => return Err("Variable should not be on the stack".to_string()),
    };
    stack.push(new);
    Ok(())
}
pub fn not(stack: &mut GMStack) -> Result<(), String> {
    let old: GMValue = stack.pop()?;
    let new: GMValue = match old {
        GMValue::Boolean(val) => GMValue::Boolean(!val),
        other => return Err(format!("Cannot bool negate {other:?} value")),
    };
    stack.push(new);
    Ok(())
}
pub fn dup(stack: &mut GMStack) -> Result<(), String> {
    let value: GMValue = stack.peek()?;
    stack.push(value);
    Ok(())
}
pub fn ret(stack: &mut GMStack) -> Result<GMValue, String> {
    let value: GMValue = stack.pop()?;
    Ok(value)
}
pub fn popz(stack: &mut GMStack) -> Result<(), String> {
    stack.pop()?;
    Ok(())
}

