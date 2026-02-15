pub mod command;

use crate::error::VMError;
use command::Command;


#[derive(Debug)]
pub struct Parser;

impl Parser {
    pub fn parse(source: &str) -> Result<Vec<Command>, VMError> {
        source
            .lines()
            .enumerate()
            .map(|(i, line)| (i + 1, line.split("//").next().unwrap_or("").trim()))
            .filter(|(_, line)| !line.is_empty())
            .map(|(line_num, line)| {
                line.parse::<Command>().map_err(|e| VMError::Parse {
                    line: line_num,
                    source: e,
                })
            })
            .collect()
    }
}
