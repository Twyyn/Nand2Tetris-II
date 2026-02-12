mod memory;
mod operation;

use crate::parser::command::{Command, OP, Segment};
use memory::Memory;
use operation::Operation;

#[derive(Debug, Default)]
pub struct CodeGen {
    label_count: u16,
}

impl CodeGen {
    pub fn new() -> Self {
        Self { label_count: 0 }
    }

    pub fn translate(&mut self, command: Command, filename: &str) -> String {
        let asm = match command {
            Command::Push { segment, index } => {
                Self::memory(segment, index, filename).push_to_asm()
            }
            Command::Pop { segment, index } => Self::memory(segment, index, filename).pop_to_asm(),
            Command::Operation { operation } => self.operation(operation).to_asm(),
            Command::Branch { branch } => todo!(),
            Command::Function { function } => todo!(),
        };

        format!("// {command} //\n{asm}")
    }

    fn memory(segment: Segment, index: u16, filename: &str) -> Memory<'_> {
        match segment {
            Segment::Constant => Memory::Constant(index),
            Segment::Local | Segment::Argument | Segment::This | Segment::That => {
                Memory::Segment(Self::base_pointer(segment), index)
            }
            Segment::Static => Memory::Static(filename, index),
            Segment::Pointer => Memory::Direct(3 + index),
            Segment::Temp => Memory::Direct(5 + index),
        }
    }

    fn operation(&mut self, operation: OP) -> Operation {
        match operation {
            OP::Add => Operation::Add,
            OP::Sub => Operation::Sub,
            OP::Neg => Operation::Neg,
            OP::Eq | OP::Gt | OP::Lt => {
                let label = self.label_count;
                self.label_count += 1;
                match operation {
                    OP::Eq => Operation::Eq(label),
                    OP::Gt => Operation::Gt(label),
                    OP::Lt => Operation::Lt(label),
                    _ => unreachable!(),
                }
            }
            OP::And => Operation::And,
            OP::Or => Operation::Or,
            OP::Not => Operation::Not,
        }
    }

    fn base_pointer(segment: Segment) -> &'static str {
        match segment {
            Segment::Local => "LCL",
            Segment::Argument => "ARG",
            Segment::This => "THIS",
            Segment::That => "THAT",
            _ => unreachable!(),
        }
    }
}
