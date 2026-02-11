#[derive(Debug)]
pub enum Operation {
    Add,
    Sub,
    Neg,
    Not,
    Eq(u16),
    Gt(u16),
    Lt(u16),
    And,
    Or,
}

fn binary_op(op: &str) -> String {
    format!(
        "\
        @SP\n \
        AM=M-1\n \
        D=M\n \
        A=A-1\n \
        M=M{op}D\n\
        "
    )
}

fn unary_op(op: &str) -> String {
    format!(
        "\
        @SP\n \
        A=M-1\n \
        M={op}M\n\
        "
    )
}

fn comparison_asm(prefix: &str, jump: &str, n: u16) -> String {
    format!(
        "\
        @SP\n \
        AM=M-1\n \
        D=M\n\
        @SP\n \
        AM=M-1\n \
        D=M-D\n\
        @{prefix}_TRUE_{n}\n \
        D;{jump}\n\
        @SP\n \
        A=M\n \
        M=0\n\
        @{prefix}_END_{n}\n \
        0;JMP\n\
        \t({prefix}_TRUE_{n})\n\
        @SP\n \
        A=M\n \
        M=-1\n\
        \t({prefix}_END_{n})\n\
        @SP\n \
        M=M+1\n\
        "
    )
}

impl Operation {
    pub fn to_asm(&self) -> String {
        match self {
            Self::Add => binary_op("+"),
            Self::Sub => binary_op("-"),
            Self::And => binary_op("&"),
            Self::Or => binary_op("|"),
            Self::Neg => unary_op("-"),
            Self::Not => unary_op("!"),
            Self::Eq(n) => comparison_asm("EQ", "JEQ", *n),
            Self::Gt(n) => comparison_asm("GT", "JGT", *n),
            Self::Lt(n) => comparison_asm("LT", "JLT", *n),
        }
    }
}
