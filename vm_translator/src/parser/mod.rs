pub mod command;
mod error;
use command::Command;
use error::ParseError;

#[derive(Debug)]
pub struct Parser;

impl Parser {
    pub fn parse(source: &str) -> Result<impl Iterator<Item = Command>, (usize, ParseError)> {
        let commands = source
            .lines()
            .enumerate()
            .map(|(i, line)| (i + 1, line.split("//").next().unwrap_or("").trim()))
            .filter(|(_, line)| !line.is_empty())
            .map(|(line_num, line)| line.parse::<Command>().map_err(|e| (line_num, e)))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(commands.into_iter())
    }
}
