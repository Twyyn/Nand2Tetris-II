mod arithmetic;
mod branching;
mod functions;
mod memory;

use crate::Write;
use crate::parser::command::Function;
use crate::{codegen::functions::translate_function, parser::command::Command};
use arithmetic::translate_arithmetic;
use branching::translate_branch;
use memory::{translate_pop, translate_push};
use std::io::Result;

#[derive(Debug, Default)]
pub struct CodeGen {
    label_count: u16,
    current_function: Option<String>,
}

impl CodeGen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn translate<W: Write>(
        &mut self,
        writer: &mut impl Write,
        command: Command,
        filename: &str,
    ) -> Result<()> {
        match command {
            Command::Push { segment, index } => {
                translate_push(writer, segment, index, filename)
            }
            Command::Pop { segment, index } => {
                translate_pop(writer, segment, index, filename)
            }
            Command::Arithmetic(operation) => {
                let label = self.next_label();
                translate_arithmetic(writer, operation, label)
            }
            Command::Branching(branch) => {
                    translate_branch(writer, branch, self.current_function.as_deref().unwrap_or("GLOBAL"))
            }
            Command::Function(function) => {
                if let Function::Declare { ref name, .. } = function {
                    self.current_function = Some(name.clone());
                }
                let label = self.next_label();
                translate_function(writer, function, label)
            }
        }
    }

    pub fn emit_bootstrap(&mut self, writer: &mut impl Write) -> Result<()> {
        let label = self.next_label();
        write!(writer,
            "// Bootstrap\n\
             @256\n\
             D=A\n\
             @SP\n\
             M=D\n"
        )?;
        translate_function(writer,
            Function::Call {
                name: "Sys.init".to_string(),
                arg_count: 0,
            },
            label,
        )
    }

    fn next_label(&mut self) -> u16 {
        let n = self.label_count;
        self.label_count += 1;
        n
    }
}
