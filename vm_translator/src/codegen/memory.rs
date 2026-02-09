use indoc::formatdoc;

pub enum Memory<'a> {
    Constant(u16),
    Segment(&'a str, u16),
    Static(&'a str, u16),
    Direct(u16), // Pointer & Temp
}

impl Memory<'_> {
    pub fn push_to_asm(&self) -> String {
        match self {
            Self::Constant(index) => formatdoc! {"
                @{index}
                D=A
                @SP
                A=M
                M=D
                @SP
                M=M+1
            "},
            Self::Segment(segment, index) => formatdoc! {"
                @{segment}
                D=M
                @{index}
                A=D+A
                D=M
                @SP
                A=M
                M=D
                @SP
                M=M+1
            "},
            Self::Static(filename, index) => formatdoc! {"
                @{filename}.{index}
                D=M
                @SP
                A=M
                M=D
                @SP
                M=M+1
            "},
            Self::Direct(addr) => formatdoc! {"
                @R{addr}
                D=M
                @SP
                A=M
                M=D
                @SP
                M=M+1
            "},
        }
    }

    pub fn pop_to_asm(&self) -> String {
        match self {
            Self::Segment(segment, index) => formatdoc! {"
                @{segment}
                D=M
                @{index}
                D=D+A
                @R13
                M=D
                @SP
                AM=M-1
                D=M
                @R13
                A=M
                M=D
            "},
            Self::Static(filename, index) => formatdoc! {"
                @SP
                AM=M-1
                D=M
                @{filename}.{index}
                M=D
            "},
            Self::Direct(addr) => formatdoc! {"
                @SP
                AM=M-1
                D=M
                @R{addr}
                M=D
            "},
            _ => unreachable!(),
        }
    }
}
