mod codegen;
pub mod error;
mod parser;
use codegen::CodeGen;
use error::VMError;
use parser::{Parser, command::Command};
use std::fs;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

fn path_is_vm_file(file: &Path) -> bool {
    file.is_file() && file.extension().is_some_and(|ext| ext == "vm")
}

fn get_vm_files(dir: &Path) -> Result<Vec<PathBuf>, VMError> {
    Ok(fs::read_dir(dir)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            path_is_vm_file(&path).then_some(path)
        })
        .collect())
}

fn filename_to_str(file_path: &Path) -> Result<String, VMError> {
    Ok(file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(VMError::InvalidInput("Invalid filename".to_string()))?
        .to_string())
}

fn output_path_from_dir(dir_path: &Path) -> Result<PathBuf, VMError> {
    dir_path
        .file_name()
        .map(|name| dir_path.join(name).with_extension("asm"))
        .ok_or(VMError::InvalidInput("Invalid directory name".to_string()))
}

#[derive(Debug)]
pub struct VMTranslator {
    file_commands: Vec<(String, Vec<Command>)>,
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
    pub fn new(input_path: &str) -> Result<Self, VMError> {
        let input_path = Path::new(input_path);

        let (vm_files, output_path) = match input_path {
            _ if input_path.is_dir() => {
                let vm_files = get_vm_files(input_path)?;
                let output_path = output_path_from_dir(input_path)?;

                (vm_files, output_path)
            }
            _ if path_is_vm_file(input_path) => {
                let vm_files = vec![input_path.into()];
                let output_path = input_path.with_extension("asm");

                (vm_files, output_path)
            }
            _ => {
                return Err(VMError::InvalidInput(
                    "File must have .vm extension".to_string(),
                ));
            }
        };

        if vm_files.is_empty() {
            return Err(VMError::InvalidInput("No .vm files found".to_string()));
        }

        let mut file_commands: Vec<(String, Vec<Command>)> = Vec::new();
        for file in &vm_files {
            let source = fs::read_to_string(file)?;
            let filename = filename_to_str(file)?;
            let commands = Parser::parse(&source)?;
            file_commands.push((filename, commands));
        }

        Ok(Self {
            file_commands,
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

        for (filename, commands) in self.file_commands {
            writeln!(writer, "// Filename: {filename}.asm")?;
            for command in commands {
                let asm = codegen.translate(command, &filename);
                writeln!(writer, "{asm}")?;
            }
        }
        writer.flush()?;

        Ok(())
    }
}
