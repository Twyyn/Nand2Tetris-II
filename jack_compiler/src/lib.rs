pub mod error;
pub mod lexer;
pub mod token;

use error::CompilerError;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

pub const JACK_INT_MAX: u32 = 32767;

#[derive(Debug)]
pub struct JackCompiler {
    pub source: Vec<Source>,
}

#[derive(Debug)]
pub struct Source {
    file_name: String,
    pub contents: String,
}

impl JackCompiler {
    pub fn new(source: &str) -> Result<Self, CompilerError> {
        let source = Path::new(source);

        let jack_files = match source {
            _ if source.is_dir() => Self::get_jack_files(source)?,

            _ if Self::is_jack_file(source) => {
                vec![source.into()]
            }

            _ => return Err(CompilerError::InvalidPath),
        };

        let source = jack_files
            .into_iter()
            .map(|path| {
                let file_name = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                let contents = fs::read_to_string(&path).map_err(CompilerError::Io)?;
                Ok(Source {
                    file_name,
                    contents,
                })
            })
            .collect::<Result<Vec<_>, CompilerError>>()?;

        Ok(Self { source })
    }

    // --- Filesystem Helpers ---

    fn is_jack_file(source: &Path) -> bool {
        source.is_file() && source.extension().is_some_and(|ext| ext == "jack")
    }

    fn get_jack_files(source: &Path) -> Result<Vec<PathBuf>, CompilerError> {
        let files: Vec<PathBuf> = fs::read_dir(source)?
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                Self::is_jack_file(&path).then_some(path)
            })
            .collect();

        if files.is_empty() {
            return Err(CompilerError::NoJackFiles);
        }

        Ok(files)
    }
}
