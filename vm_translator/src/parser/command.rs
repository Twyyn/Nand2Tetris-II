pub enum Command {
    Push{ segment: Segment, index: u64 },
    Pop,
}

pub enum Segment {
    Constant,
    Local,
}
