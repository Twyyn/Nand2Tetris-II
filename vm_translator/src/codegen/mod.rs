mod arithmetic;
mod bootstrap;
mod branching;
mod functions;
mod memory;

use crate::parser::command::Function;
use crate::{codegen::functions::translate_function, parser::command::Command};
use arithmetic::translate_arithmetic;
use branching::translate_branch;
use memory::{translate_pop, translate_push};

#[derive(Debug, Default)]
pub struct CodeGen {
    label_count: u16,
    current_function: String,
}

impl CodeGen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn translate(&mut self, command: Command, filename: &str, label: UniqueLabel) -> String {
        match command {
            Command::Push { segment, index } => translate_push(segment, index, filename),
            Command::Pop { segment, index } => translate_pop(segment, index, filename),
            Command::Arithmetic(operation) => {
                let label = self.next_label();
                translate_arithmetic(operation, label.id)
            }
            Command::Branching(branch) => translate_branch(branch, &self.current_function),
            Command::Function(function) => {
                let label = self.next_label();
                match function {
                    Function::Declare { name, var_count } => {
                        self.current_function = name.clone();
                        translate_function(Function::Declare { name, var_count }, label)
                    }
                    other => translate_function(other, label),
                }
            }
        }
    }

    fn next_label(&mut self) -> UniqueLabel {
        let id = self.label_count;
        self.label_count += 1;
        UniqueLabel { id }
    }
}

#[derive(Debug)]
pub struct UniqueLabel {
    id: u16,
}

impl UniqueLabel {
    pub fn prefixed(&self, prefix: &str) -> String {
        format!("{prefix}_{}", self.id)
    }

    pub fn return_label(&self, fn_name: &str) -> String {
        format!("{fn_name}$ret.{}", self.id)
    }
}
