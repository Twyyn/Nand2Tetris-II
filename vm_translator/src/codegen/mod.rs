mod bootstrap;
mod branching;
mod functions;
mod memory;
mod arithmetic;

use crate::parser::command::Function;
use crate::{
    codegen::functions::translate_function,
    parser::command::{Command, Operation},
};
use bootstrap::bootstrap;
use branching::translate_branch;
pub use memory::{translate_pop, translate_push};
use arithmetic::translate_arithmetic;

pub struct Bootstrap {}

#[derive(Debug, Default)]
pub struct CodeGen {
    label_count: u16,
    current_funtion: String,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            label_count: 0,
            current_funtion: String::new(),
        }
    }

    pub fn translate(&mut self, command: Command, filename: &str) -> String {
        match command {
            Command::Push { segment, index } => translate_push(segment, index, filename),
            Command::Pop { segment, index } => translate_pop(segment, index, filename),
            Command::Arithmetic(operation) => {
                let label = self.next_label();
                translate_arithmetic(operation, label)
            }
            Command::Branching(branch) => translate_branch(branch, &self.current_funtion),
            Command::Function(function) => {
                self.current_funtion = function.to_string();
                let label = self.next_label();
                translate_function(function, label)
            }
        }
    }

    fn next_label(&mut self) -> u16 {
        let label = self.label_count;
        self.label_count += 1;
        label
    }
}
