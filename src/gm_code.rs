use libgm::gm::{GMCode, GMDataType, GMInstruction, GMOpcode, GMValue};
use crate::App;

#[derive(Debug)]
pub struct GMStack {
    pub items: Vec<GMValue>,   // GMValue can be anything except a Variable
}
impl GMStack {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    fn push(&mut self, value: GMValue) {
        self.items.push(value);
    }
    fn pop(&mut self) -> Result<GMValue, String> {
        self.items.pop()
            .ok_or_else(|| "Could not pop value from stack because it is empty".to_string())
    }
    fn peek(&self) -> Result<GMValue, String> {
        self.items.last()
            .map(|i| i.clone())
            .ok_or_else(|| "Could not peek value from stack because it is empty".to_string())
    }
}


impl App {
    pub fn run_code(&mut self, code: &GMCode) -> Result<(), String> {
        for instruction in &code.instructions {
            self.run_instruction(instruction)?;
        }
        Ok(())
    }

    fn run_instruction(&mut self, instruction: &GMInstruction) -> Result<Option<GMValue>, String> {
        match instruction {
            GMInstruction::SingleType(instr) => {
                match instr.opcode {
                    GMOpcode::Neg => neg(&mut self.stack)?,
                    GMOpcode::Not => not(&mut self.stack)?,
                    GMOpcode::Dup => dup(&mut self.stack)?,
                    GMOpcode::Ret => return Ok(Some(ret(&mut self.stack)?)),
                    GMOpcode::Exit => return Ok(None),
                    GMOpcode::Popz => popz(&mut self.stack)?,
                    other => return Err(format!("Invalid Single Type Instruction Opcode {other:?}"))
                }
            }

            GMInstruction::DoubleType(instr) => {
                match instr.opcode {
                    GMOpcode::Conv => ,
                    GMOpcode::Mul => ,
                    GMOpcode::Div => ,
                    GMOpcode::Rem => ,
                    GMOpcode::Mod => ,
                    GMOpcode::Add => ,
                    GMOpcode::Sub => ,
                    GMOpcode::And => ,
                    GMOpcode::Or => ,
                    GMOpcode::Xor => ,
                    GMOpcode::Shl => ,
                    GMOpcode::Shr => ,
                    other => return Err(format!("Invalid Double Type Instruction Opcode {other:?}"))
                }
            }
            
            GMInstruction::Comparison(_) => {}
            GMInstruction::Goto(_) => {}
            GMInstruction::Pop(_) => {}
            GMInstruction::Push(_) => {}
            GMInstruction::Call(_) => {}
            GMInstruction::Break(_) => {}
        }
        Ok(())
    }
}



// single type instructions
fn neg(stack: &mut GMStack) -> Result<(), String> {
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
fn not(stack: &mut GMStack) -> Result<(), String> {
    let old: GMValue = stack.pop()?;
    let new: GMValue = match old {
        GMValue::Boolean(val) => GMValue::Boolean(!val),
        other => return Err(format!("Cannot bool negate {other:?} value")),
    };
    stack.push(new);
    Ok(())
}
fn dup(stack: &mut GMStack) -> Result<(), String> {
    let value: GMValue = stack.peek()?;
    stack.push(value);
    Ok(())
}
fn ret(stack: &mut GMStack) -> Result<GMValue, String> {
    let value: GMValue = stack.pop()?;
    Ok(value)
}
fn popz(stack: &mut GMStack) -> Result<(), String> {
    stack.pop()?;
    Ok(())
}


// double type instructions
fn conv(stack: &mut GMStack, target_data_type: GMDataType) -> Result<(), String> {
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


