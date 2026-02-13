use crate::parser::command::Op;

fn binary_op(op: &str) -> String {
    format!(
        "\
        @SP\n\
        AM=M-1\n\
        D=M\n\
        A=A-1\n\
        M=M{op}D\n\
        "
    )
}

fn unary_op(op: &str) -> String {
    format!(
        "\
        @SP\n\
        A=M-1\n\
        M={op}M\n\
        "
    )
}

fn comparison_asm(prefix: &str, jump: &str, n: u16) -> String {
    format!(
        "\
        @SP\n\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        AM=M-1\n\
        D=M-D\n\
        @{prefix}_TRUE_{n}\n\
        D;{jump}\n\
        @SP\n\
        A=M\n\
        M=0\n\
        @{prefix}_END_{n}\n\
        0;JMP\n\
        ({prefix}_TRUE_{n})\n\
        @SP\n\
        A=M\n\
        M=-1\n\
        ({prefix}_END_{n})\n\
        @SP\n\
        M=M+1\n\
        "
    )
}

pub fn op_to_asm(op: Op, label: u16) -> String {
    match op {
        Op::Add => binary_op("+"),
        Op::Sub => binary_op("-"),
        Op::And => binary_op("&"),
        Op::Or => binary_op("|"),
        Op::Neg => unary_op("-"),
        Op::Not => unary_op("!"),
        Op::Eq => comparison_asm("EQ", "JEQ", label),
        Op::Gt => comparison_asm("GT", "JGT", label),
        Op::Lt => comparison_asm("LT", "JLT", label),
    }
}
