mod branching;
mod functions;
mod memory;
mod operations;

use crate::parser::command::{Command, Operation};
use branching::translate_branch;
pub use memory::{translate_pop, translate_push};
use operations::translate_arithmetic;

#[derive(Debug, Default)]
pub struct CodeGen {
    label_count: u16,
}

impl CodeGen {
    pub fn new() -> Self {
        Self { label_count: 0 }
    }

    pub fn translate(&mut self, command: Command, filename: &str) -> String {
        let comment = format!("// {command}");
        let asm = match command {
            Command::Push { segment, index } => translate_push(segment, index, filename),
            Command::Pop { segment, index } => translate_pop(segment, index, filename),
            Command::Arithmetic(operation) => {
                let label = self.next_label(operation);
                translate_arithmetic(operation, label)
            }
            Command::Branching(branch) => translate_branch(branch, todo!()),
            Command::Function(function) => todo!(),
        };
        format!("{comment}\n{asm}")
    }
    fn next_label(&mut self, operation: Operation) -> u16 {
        match operation {
            Operation::Eq | Operation::Gt | Operation::Lt => {
                let label = self.label_count;
                self.label_count += 1;
                label
            }
            _ => 0,
        }
    }
}
