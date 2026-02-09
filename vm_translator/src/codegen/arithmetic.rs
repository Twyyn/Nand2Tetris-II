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
    format!(
        "@SP\n\
         AM=M-1\n\
         D=M\n\
         A=A-1\n\
         M=M{op}D\n"
    )
}

fn unary_op(op: &str) -> String {
    format!(
        "@SP\n\
         A=M-1\n\
         M={op}M\n"
    )
}

fn comparison_asm(prefix: &str, jump: &str, label_count: u16) -> String {
    format!(
        "@SP\n\
         AM=M-1\n\
         D=M\n\
         @SP\n\
         AM=M-1\n\
         D=M-D\n\
         \n\
         @{prefix}_TRUE_{label_count}\n\
         D;{jump}\n\
         \n\
         @SP\n\
         A=M\n\
         M=0\n\
         @{prefix}_END_{label_count}\n\
         0;JMP\n\
         \n\
         ({prefix}_TRUE_{label_count})\n\
         @SP\n\
         A=M\n\
         M=-1\n\
         \n\
         ({prefix}_END_{label_count})\n\
         @SP\n\
         M=M+1\n"
    )
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
