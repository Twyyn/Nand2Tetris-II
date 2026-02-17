use crate::{codegen::UniqueLabel, parser::command::Operation};


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
    let (xpos_yneg_result, xneg_ypos_result) = match jump {
        "JGT" => ("-1", "0"),
        "JLT" => ("0", "-1"),
        "JEQ" => ("0", "0"),
        _ => unreachable!(),
    };

    format!(
        // Pop y into R14
        "@SP\n\
         AM=M-1\n\
         D=M\n\
         @R14\n\
         M=D\n\
         // Pop x into R13\n\
         @SP\n\
         AM=M-1\n\
         D=M\n\
         @R13\n\
         M=D\n\
         // Check sign of x\n\
         @{prefix}_X_NEG_{n}\n\
         D;JLT\n\
         // x >= 0: check y\n\
         @R14\n\
         D=M\n\
         @{prefix}_DIFF_XPOS_{n}\n\
         D;JLT\n\
         // Both non-negative: safe subtract\n\
         @{prefix}_SAFE_{n}\n\
         0;JMP\n\
         ({prefix}_X_NEG_{n})\n\
         // x < 0: check y\n\
         @R14\n\
         D=M\n\
         @{prefix}_DIFF_XNEG_{n}\n\
         D;JGE\n\
         // Both negative: safe subtract\n\
         ({prefix}_SAFE_{n})\n\
         @R13\n\
         D=M\n\
         @R14\n\
         D=D-M\n\
         @{prefix}_TRUE_{n}\n\
         D;{jump}\n\
         @SP\n\
         A=M\n\
         M=0\n\
         @{prefix}_END_{n}\n\
         0;JMP\n\
         ({prefix}_DIFF_XPOS_{n})\n\
         // x >= 0, y < 0: x is greater\n\
         @SP\n\
         A=M\n\
         M={xpos_yneg_result}\n\
         @{prefix}_END_{n}\n\
         0;JMP\n\
         ({prefix}_DIFF_XNEG_{n})\n\
         // x < 0, y >= 0: y is greater\n\
         @SP\n\
         A=M\n\
         M={xneg_ypos_result}\n\
         @{prefix}_END_{n}\n\
         0;JMP\n\
         ({prefix}_TRUE_{n})\n\
         @SP\n\
         A=M\n\
         M=-1\n\
         ({prefix}_END_{n})\n\
         @SP\n\
         M=M+1\n"
    )
}

pub fn translate_arithmetic(operation: Operation, label: u16) -> String {
    match operation {
        Operation::Add => binary_op("+"),
        Operation::Sub => binary_op("-"),
        Operation::And => binary_op("&"),
        Operation::Or => binary_op("|"),
        Operation::Neg => unary_op("-"),
        Operation::Not => unary_op("!"),
        Operation::Eq => comparison_asm("EQ", "JEQ", label),
        Operation::Gt => comparison_asm("GT", "JGT", label),
        Operation::Lt => comparison_asm("LT", "JLT", label),
    }
}
