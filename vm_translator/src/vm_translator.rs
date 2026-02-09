use crate::codegen::CodeGen;
use crate::parser::Parser;
use crate::parser::command::Command;
use std::fs;

#[derive(Debug)]
#[allow(unused)]
pub struct VMTranslator {
    filename: String,
    commands: Vec<Command>,
}

impl VMTranslator {
    pub fn new(filepath: &str) -> Result<Self, String> {
        let filename = std::path::Path::new(filepath)
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid filename")?
            .to_string();

        let source =
            fs::read_to_string(filepath).map_err(|e| format!("Failed to read {filepath}: {e}"))?;

        let commands = Parser::parse(&source)
            .map_err(|(line, e)| format!("Parse error at line {line}: {e}"))?;

        Ok(Self { filename, commands })
    }

    pub fn run(self) {
        let mut codegen = CodeGen::new();

        for command in self.commands {
            println!("{}", codegen.translate(command, &self.filename));
        }
    }
}
