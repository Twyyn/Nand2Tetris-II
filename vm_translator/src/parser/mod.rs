pub mod command;
pub mod errors;
use command::Command;
use errors::ParseError;

#[derive(Debug)]
pub struct Parser;

impl Parser {
    pub fn parse(source: &str) -> Result<Vec<Command>, (usize, ParseError)> {
        source
            .lines()
            .enumerate()
            .map(|(i, line)| (i + 1, line.split("//").next().unwrap_or("").trim()))
            .filter(|(_, line)| !line.is_empty())
            .map(|(line_num, line)| line.parse().map_err(|e| (line_num, e)))
            .collect()
    }
}
