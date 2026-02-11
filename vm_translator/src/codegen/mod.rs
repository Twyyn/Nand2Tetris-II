mod memory;
mod operation;

use crate::{
    codegen::{memory::Memory, operation::Operation},
    parser::command::{Command, Op, Segment},
};

#[derive(Debug, Default)]
pub struct CodeGen {
    label_count: u16,
}

impl CodeGen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn translate(&mut self, command: Command, filename: &str) -> String {
        let asm = match command {
            Command::Push { segment, index } => {
                Self::memory(segment, index, filename).push_to_asm()
            }
            Command::Pop { segment, index } => Self::memory(segment, index, filename).pop_to_asm(),
            Command::Operation(operation) => self.operation(operation).to_asm(),
        };

        format!("// {command} //\n{asm}")
    }

    fn memory<'a>(segment: Segment, index: u16, filename: &'a str) -> Memory<'a> {
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

    fn operation(&mut self, operation: Op) -> Operation {
        match operation {
            Op::Add => Operation::Add,
            Op::Sub => Operation::Sub,
            Op::Neg => Operation::Neg,
            Op::Eq | Op::Gt | Op::Lt => {
                let label = self.label_count;
                self.label_count += 1;
                match operation {
                    Op::Eq => Operation::Eq(label),
                    Op::Gt => Operation::Gt(label),
                    Op::Lt => Operation::Lt(label),
                    _ => unreachable!(),
                }
            }
            Op::And => Operation::And,
            Op::Or => Operation::Or,
            Op::Not => Operation::Not,
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
