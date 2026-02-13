use crate::parser::command::OperationCommand;

fn binary_op(operation: &str) -> String {
    format!(
        "\
        @SP\n\
        AM=M-1\n\
        D=M\n\
        A=A-1\n\
        M=M{operation}D\n\
        "
    )
}

fn unary_op(operation: &str) -> String {
    format!(
        "\
        @SP\n\
        A=M-1\n\
        M={operation}M\n\
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

pub fn compile_op(operation_command: OperationCommand, label: u16) -> String {
    match operation_command {
        OperationCommand::Add => binary_op("+"),
        OperationCommand::Sub => binary_op("-"),
        OperationCommand::And => binary_op("&"),
        OperationCommand::Or => binary_op("|"),
        OperationCommand::Neg => unary_op("-"),
        OperationCommand::Not => unary_op("!"),
        OperationCommand::Eq => comparison_asm("EQ", "JEQ", label),
        OperationCommand::Gt => comparison_asm("GT", "JGT", label),
        OperationCommand::Lt => comparison_asm("LT", "JLT", label),
    }
}
