mod memory;
mod operation;

use crate::parser::command::{Command, Op};
use memory::{pop_to_asm, push_to_asm};
use operation::op_to_asm;

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
            Command::Push { segment, index } => push_to_asm(segment, index, filename),
            Command::Pop { segment, index } => pop_to_asm(segment, index, filename),
            Command::Operation(op) => {
                let label = self.next_label(op);
                op_to_asm(op, label)
            }
            Command::Branch(br) => todo!(),
            Command::Function(func) => todo!(),
        };
        format!("// {command}\n{asm}")
    }
    fn next_label(&mut self, op: Op) -> u16 {
        match op {
            Op::Eq | Op::Gt | Op::Lt => {
                let label = self.label_count;
                self.label_count += 1;
                label
            }
            _ => 0,
        }
    }
}
