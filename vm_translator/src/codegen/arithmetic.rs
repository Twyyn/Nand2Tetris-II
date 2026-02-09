use indoc::formatdoc;

pub enum Arithmetic {
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
    formatdoc! {"
        @SP
        AM=M-1
        D=M
        A=A-1
        M=M{op}D
    "}
}

fn unary_op(op: &str) -> String {
    formatdoc! {"
        @SP
        A=M-1
        M={op}M
    "}
}

fn comparison_asm(prefix: &str, jump: &str, label_count: u16) -> String {
    formatdoc! {"
        @SP
        AM=M-1
        D=M
        @SP
        AM=M-1
        D=M-D

        @{prefix}_TRUE_{label_count}
        D;{jump}

        @SP
        A=M
        M=0
        @{prefix}_END_{label_count}
        0;JMP

        ({prefix}_TRUE_{label_count})
        @SP
        A=M
        M=-1

        ({prefix}_END_{label_count})
        @SP
        M=M+1
    "}
}

impl Arithmetic {
    pub fn to_asm(&self) -> String {
        match self {
            Self::Add => binary_op("+"),
            Self::Sub => binary_op("-"),
            Self::And => binary_op("&"),
            Self::Or => binary_op("|"),
            Self::Neg => unary_op("-"),
            Self::Not => unary_op("!"),
            Self::Eq(label) => comparison_asm("EQ", "JEQ", *label),
            Self::Gt(label) => comparison_asm("GT", "JGT", *label),
            Self::Lt(label) => comparison_asm("LT", "JLT", *label),
        }
    }
}
