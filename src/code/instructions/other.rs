use libgm::gm::{GMCodeVariable, GMComparisonType, GMInstanceType, GMValue};
use crate::code::run::{Stack, Variables};

pub fn cmp(stack: &mut Stack, comparison_type: GMComparisonType) -> Result<(), String> {
    let lhs: GMValue = stack.pop()?;
    let rhs: GMValue = stack.pop()?;
    
    let result: bool = match (&lhs, &rhs) {
        (GMValue::Double(a), GMValue::Double(b)) => compare(a, b, comparison_type),
        (GMValue::Float(a), GMValue::Float(b)) => compare(a, b, comparison_type),
        (GMValue::Int16(a), GMValue::Int16(b)) => compare(a, b, comparison_type),
        (GMValue::Int32(a), GMValue::Int32(b)) => compare(a, b, comparison_type),
        (GMValue::Int64(a), GMValue::Int64(b)) => compare(a, b, comparison_type),
        (GMValue::Boolean(a), GMValue::Boolean(b)) => compare(a, b, comparison_type),
        _ => return Err(format!("Cannot compare {lhs:?} and {rhs:?}"))
    };

    stack.push(GMValue::Boolean(result));
    Ok(())
}

fn compare<T: PartialOrd>(a: T, b: T, comparison_type: GMComparisonType) -> bool {
    match comparison_type {
        GMComparisonType::LT => a < b,
        GMComparisonType::LTE => a <= b,
        GMComparisonType::EQ => a == b,
        GMComparisonType::NEQ => a != b,
        GMComparisonType::GTE => a >= b,
        GMComparisonType::GT => a > b,
    }
}



/// returns whether to jump
pub fn bt(stack: &mut Stack) -> Result<bool, String> {
    let value: GMValue = stack.pop()?;
    match value {
        GMValue::Boolean(boolean) => Ok(boolean),
        other => Err(format!("Expected boolean for jump condition, got {other:?}"))
    }
}

/// returns whether to jump
pub fn bf(stack: &mut Stack) -> Result<bool, String> {
    let value: GMValue = stack.pop()?;
    match value {
        GMValue::Boolean(boolean) => Ok(!boolean),
        other => Err(format!("Expected boolean for jump condition, got {other:?}"))
    }
}


pub fn pop(
    variables: &mut Variables,
    code_index: usize,
    object_index: usize,
    stack: &mut Stack,
    instance_type: &GMInstanceType,
    destination: &GMCodeVariable,
) -> Result<(), String> {
    let value: GMValue = stack.pop()?;

    match instance_type {
        GMInstanceType::Instance(Some(obj)) => {
            variables.instances.insert((destination.variable.index, obj.index), value);
        }
        GMInstanceType::Instance(None) => {
            variables.instances.insert((destination.variable.index, object_index), value);
        }
        GMInstanceType::Global => {
            variables.globals.insert(destination.variable.index, value);
        }
        GMInstanceType::Local => {
            variables.locals.insert((destination.variable.index, code_index), value);
        }
        other => return Err(format!("Invalid Instance Type {other:?} while popping value {value:?}"))
    }

    Ok(())
}

