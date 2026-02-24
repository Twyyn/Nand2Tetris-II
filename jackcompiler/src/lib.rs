pub mod error;
pub mod lexer;
pub mod parser;

use crate::error::CompilerError;
use std::{
    fs::{self},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct SourceFile {
    pub name: String,
    pub contents: String,
}

#[derive(Debug)]
pub struct JackCompiler {
    pub source: Vec<SourceFile>,
    output_path: PathBuf,
}

impl JackCompiler {
    /// Create a new `JackCompiler` from a path to either a `.jack` file or a directory
    /// containing `.jack` files.
    ///
    /// # Errors
    ///
    /// Returns `Err(CompilerError::InvalidInput(_))` if the provided path is not a `.jack`
    /// file nor a directory containing `.jack` files, or if no `.jack` files are found.
    /// Returns other `CompilerError` variants if filesystem operations fail (e.g. reading
    /// directories or files).
    pub fn new(source_path: &str) -> Result<Self, CompilerError> {
        let source_path = Path::new(source_path);

        let (jack_files, output_path) = match source_path {
            _ if source_path.is_dir() => {
                let jack_files = get_jack_files(source_path)?;
                let output_path = output_path_from_dir(source_path)?;

                (jack_files, output_path)
            }
            _ if is_jack_file(source_path) => {
                let jack_files = vec![source_path.into()];
                let output_path = source_path.with_extension("vm");

                (jack_files, output_path)
            }
            _ => {
                return Err(CompilerError::InvalidInput(
                    "File must have .jack extension".to_string(),
                ));
            }
        };

        if jack_files.is_empty() {
            return Err(CompilerError::InvalidInput(
                "No .jack files found".to_string(),
            ));
        }

        let mut source: Vec<SourceFile> = Vec::new();
        for file in jack_files {
            let name = extract_name(&file)?;
            let contents = fs::read_to_string(file)?;
            source.push(SourceFile { name, contents });
        }

        Ok(Self {
            source,
            output_path,
        })
    }
}

fn is_jack_file(file: &Path) -> bool {
    file.is_file() && file.extension().is_some_and(|ext| ext == "jack")
}

fn get_jack_files(dir: &Path) -> Result<Vec<PathBuf>, CompilerError> {
    Ok(fs::read_dir(dir)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            is_jack_file(&path).then_some(path)
        })
        .collect())
}

fn output_path_from_dir(dir_path: &Path) -> Result<PathBuf, CompilerError> {
    dir_path
        .file_name()
        .map(|name| dir_path.join(name).with_extension("vm"))
        .ok_or(CompilerError::InvalidInput(
            "Invalid directory name".to_string(),
        ))
}

fn extract_name(file_path: &Path) -> Result<String, CompilerError> {
    Ok(file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(CompilerError::InvalidInput("Invalid filename".to_string()))?
        .to_string())
}
