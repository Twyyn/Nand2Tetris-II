mod codegen;
pub mod error;
mod parser;
use codegen::CodeGen;
use error::VMError;
use parser::{Parser, command::Command};
use std::{
    ffi::OsStr,
    fs,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct VMTranslator {
    filename: String,
    commands: Vec<Command>,
    output_path: PathBuf,
}

impl VMTranslator {
    /// Creates a new `VMTranslator` from a `.vm` file path.
    ///
    /// # Errors
    ///
    /// Returns `VMError::InvalidInput` if the file extension is wrong or filename is invalid.
    /// Returns `VMError::IO` if the file cannot be read.
    /// Returns `VMError::Parse` if any VM command is invalid.
    pub fn new(filepath: &str) -> Result<Self, VMError> {
        let filepath = Path::new(filepath);

        if filepath.extension() != Some(OsStr::new("vm")) {
            return Err(VMError::InvalidInput(
                "File must have .vm extension".to_string(),
            ));
        }

        let filename = filepath
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or(VMError::InvalidInput("Invalid filename".to_string()))?
            .to_string();

        let source = fs::read_to_string(filepath)?;
        let commands = Parser::parse(&source)?;
        let output_path = filepath.with_extension("asm");

        Ok(Self {
            filename,
            commands,
            output_path,
        })
    }

    /// Translates the VM commands to assembly and writes to the output file.
    ///
    /// # Errors
    ///
    /// Returns an `VMError` if writing to the output file fails.
    pub fn run(self) -> Result<(), VMError> {
        let mut codegen = CodeGen::new();

        let file = fs::File::create(&self.output_path)?;
        let mut writer = BufWriter::new(file);

        for command in self.commands {
            let asm = codegen.translate(command, &self.filename);
            writeln!(writer, "{asm}")?;
        }
        writer.flush()?;

        Ok(())
    }
}
