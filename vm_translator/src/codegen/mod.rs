mod arithmetic;
mod memory;

use crate::{
    codegen::{arithmetic::Arithmetic, memory::Memory},
    parser::command::{ArithmeticCommand, Command, Segment},
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
            Command::Arithmetic(operation) => self.arithmetic(operation).to_asm(),
        };

        format!("// {command}\n{asm}")
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

    fn arithmetic(&mut self, operation: ArithmeticCommand) -> Arithmetic {
        match operation {
            ArithmeticCommand::Add => Arithmetic::Add,
            ArithmeticCommand::Sub => Arithmetic::Sub,
            ArithmeticCommand::Neg => Arithmetic::Neg,
            ArithmeticCommand::Eq | ArithmeticCommand::Gt | ArithmeticCommand::Lt => {
                let label = self.label_count;
                self.label_count += 1;
                match operation {
                    ArithmeticCommand::Eq => Arithmetic::Eq(label),
                    ArithmeticCommand::Gt => Arithmetic::Gt(label),
                    ArithmeticCommand::Lt => Arithmetic::Lt(label),
                    _ => unreachable!(),
                }
            }
            ArithmeticCommand::And => Arithmetic::And,
            ArithmeticCommand::Or => Arithmetic::Or,
            ArithmeticCommand::Not => Arithmetic::Not,
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
