mod command;
use command::{Command, Segment};

pub struct Parser {
    lines: Vec<String>,
    current: usize,
}

impl Parser {
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines, current: 1 }
    }

    pub fn parse(&mut self) -> Command {
        todo!()
    }

    fn next_command() {
        todo!()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.lines.len()
    }
}
