use std::collections::HashMap;
use libgm::gm::{GMCode, GMFunction, GMInstruction, GMOpcode, GMValue};
use crate::App;
use crate::code::instructions::double_type::{add, and, conv, div, mod_, mul, or, rem, shl, shr, sub, xor};
use crate::code::instructions::other::{bf, bt, cmp, pop};
use crate::code::instructions::single_type::{dup, neg, not, popz, ret};

#[derive(Debug)]
pub struct Stack {
    pub items: Vec<GMValue>,   // GMValue can be anything except a Variable
}
impl Stack {
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


#[derive(Debug, Clone)]
pub struct Variables {
    pub globals: HashMap<usize, GMValue>,               // key: variable index
    pub instances: HashMap<(usize, usize), GMValue>,    // key: variable index, game object index
    pub locals: HashMap<(usize, usize), GMValue>,       // key: variable index, code index. Is partially reset after execution of a code ends
}


impl App {
    pub fn run_code(&mut self, code_index: usize) -> Result<Option<GMValue>, String> {
        let code: &GMCode = &self.data.codes.codes_by_index[code_index];
        let mut i: usize = 0;
        
        while i < code.instructions.len() {
            match &code.instructions[i] {
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

                GMInstruction::Comparison(instr) => {
                    cmp(&mut self.stack, instr.comparison_type)?;
                }

                GMInstruction::Goto(instr) => {
                    match instr.opcode {
                        GMOpcode::B => i = (i as i32 + instr.jump_offset * 4) as usize,
                        GMOpcode::Bt => if bt(&mut self.stack)? { i = (i as i32 + instr.jump_offset * 4) as usize },
                        GMOpcode::Bf => if bf(&mut self.stack)? { i = (i as i32 + instr.jump_offset * 4) as usize },
                        GMOpcode::PushEnv => todo!(),
                        GMOpcode::PopEnv => todo!(),
                        other => return Err(format!("Invalid Goto Instruction Opcode {other:?}"))
                    }
                }

                GMInstruction::Pop(instr) => {
                    pop(&mut self.variables, code_index, &mut self.stack, &instr.instance_type, &instr.destination)?;
                }

                GMInstruction::Push(instr) => {
                    self.stack.push(instr.value.clone());
                }

                GMInstruction::Call(instr) => {
                    todo!()
                    // let function: &GMFunction = instr.function.resolve(&self.data.functions.functions_by_index)?;
                    // function.
                    // self.run_code()
                }

                GMInstruction::Break(instr) => {
                    todo!()
                }
            }

            i += 1;     // increment instruction counter
        }
        Ok(None)
    }


}


