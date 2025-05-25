use libgm::gm::{GMCode, GMDataType, GMInstruction, GMOpcode, GMValue};
use crate::App;
use crate::code::instructions::double_type::{add, and, conv, div, mod_, mul, or, rem, shl, shr, sub, xor};
use crate::code::instructions::single_type::{dup, neg, not, popz, ret};

#[derive(Debug)]
pub struct GMStack {
    pub items: Vec<GMValue>,   // GMValue can be anything except a Variable
}
impl GMStack {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn push(&mut self, value: GMValue) {
        self.items.push(value);
    }
    pub fn pop(&mut self) -> Result<GMValue, String> {
        self.items.pop()
            .ok_or_else(|| "Could not pop value from stack because it is empty".to_string())
    }
    pub fn peek(&self) -> Result<GMValue, String> {
        self.items.last()
            .map(|i| i.clone())
            .ok_or_else(|| "Could not peek value from stack because it is empty".to_string())
    }
}


impl App {
    pub fn run_code(&mut self, code: &GMCode) -> Result<Option<GMValue>, String> {
        for instruction in &code.instructions {
            if let Some(return_value) = self.run_instruction(instruction)? {
                return Ok(Some(return_value))
            }
        }
        Ok(None)
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
                    GMOpcode::Conv => conv(&mut self.stack, instr.type2)?,
                    GMOpcode::Mul => mul(&mut self.stack)?,
                    GMOpcode::Div => div(&mut self.stack)?,
                    GMOpcode::Rem => rem(&mut self.stack)?,
                    GMOpcode::Mod => mod_(&mut self.stack)?,
                    GMOpcode::Add => add(&mut self.stack)?,
                    GMOpcode::Sub => sub(&mut self.stack)?,
                    GMOpcode::And => and(&mut self.stack)?,
                    GMOpcode::Or => or(&mut self.stack)?,
                    GMOpcode::Xor => xor(&mut self.stack)?,
                    GMOpcode::Shl => shl(&mut self.stack)?,
                    GMOpcode::Shr => shr(&mut self.stack)?,
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
        Ok(None)
    }
}


