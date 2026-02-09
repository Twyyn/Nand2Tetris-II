pub enum Memory<'a> {
    Constant(u16),
    Segment(&'a str, u16),
    Static(&'a str, u16),
    Direct(u16), // Pointer & Temp
}

impl Memory<'_> {
    pub fn push_to_asm(&self) -> String {
        match self {
            Self::Constant(index) => format!(
                "@{index}\n\
                 D=A\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n"
            ),
            Self::Segment(segment, index) => format!(
                "@{segment}\n\
                 D=M\n\
                 @{index}\n\
                 A=D+A\n\
                 D=M\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n"
            ),
            Self::Static(filename, index) => format!(
                "@{filename}.{index}\n\
                 D=M\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n"
            ),
            Self::Direct(addr) => format!(
                "@R{addr}\n\
                 D=M\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n"
            ),
        }
    }

    pub fn pop_to_asm(&self) -> String {
        match self {
            Self::Segment(segment, index) => format!(
                "@{segment}\n\
                 D=M\n\
                 @{index}\n\
                 D=D+A\n\
                 @R13\n\
                 M=D\n\
                 @SP\n\
                 AM=M-1\n\
                 D=M\n\
                 @R13\n\
                 A=M\n\
                 M=D\n"
            ),
            Self::Static(filename, index) => format!(
                "@SP\n\
                 AM=M-1\n\
                 D=M\n\
                 @{filename}.{index}\n\
                 M=D\n"
            ),
            Self::Direct(addr) => format!(
                "@SP\n\
                 AM=M-1\n\
                 D=M\n\
                 @R{addr}\n\
                 M=D\n"
            ),
            _ => unreachable!(),
        }
    }
}
