#[derive(Debug)]
pub enum Memory<'a> {
    Constant(u16),
    Segment(&'a str, u16),
    Static(&'a str, u16),
    Direct(u16),
}

const PUSH_D: &str = "\
    @SP\n \
    A=M\n \
    M=D\n\
    @SP\n \
    M=M+1\n\
    ";

const POP_TO_D: &str = "\
    @SP\n \
    AM=M-1\n \
    D=M\n\
    ";

impl<'a> Memory<'a> {
    pub fn push_to_asm(&self) -> String {
        match self {
            Self::Constant(index) => format!(
                "\
                @{index}\n \
                 D=A\n\
                 {PUSH_D}\
                 "
            ),
            Self::Segment(segment, index) => format!(
                "\
                @{segment}\n \
                D=M\n\
                @{index}\n \
                A=D+A\n \
                D=M\n\
                {PUSH_D}\
                "
            ),
            Self::Static(filename, index) => format!(
                "\
                @{filename}.{index}\n \
                D=M\n\
                {PUSH_D}\
                "
            ),
            Self::Direct(addr) => format!(
                "\
                @R{addr}\n \
                D=M\n\
                {PUSH_D}\
                "
            ),
        }
    }

    pub fn pop_to_asm(&self) -> String {
        match self {
            Self::Segment(segment, index) => format!(
                "\
                @{segment}\n \
                D=M\n\
                @{index}\n \
                D=D+A\n\
                @R13\n \
                M=D\n\
                {POP_TO_D}\
                @R13\n \
                A=M\n \
                M=D\n\
                "
            ),
            Self::Static(filename, index) => format!(
                "\
                {POP_TO_D}\
                 @{filename}.{index}\n \
                 M=D\n\
                 "
            ),
            Self::Direct(addr) => format!(
                "\
                {POP_TO_D}\
                @R{addr}\n \
                M=D\n\
                "
            ),
            _ => unreachable!(),
        }
    }
}
