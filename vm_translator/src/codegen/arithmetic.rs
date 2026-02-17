use crate::parser::command::Operation;
use std::io::{self, Write};

fn binary_op(writer: &mut impl Write, operation: &str) -> io::Result<()> {
    // Replaced format! with write! and return the Result
    write!(
        writer,
        "@SP\n\
         AM=M-1\n\
         D=M\n\
         A=A-1\n\
         M=M{operation}D\n"
    )
}

fn unary_op(writer: &mut impl Write, operation: &str) -> io::Result<()> {
    write!(
        writer,
        "@SP\n\
         A=M-1\n\
         M={operation}M\n"
    )
}

fn comparison_asm(writer: &mut impl Write, prefix: &str, jump: &str, n: u16) -> io::Result<()> {
    let (x, y) = match jump {
        "JGT" => ("-1", "0"),
        "JLT" => ("0", "-1"),
        "JEQ" => ("0", "0"),
        _ => unreachable!(),
    };

    write!(
        writer,
        "@SP\n\
         AM=M-1\n\
         D=M\n\
         @R14\n\
         M=D\n\
         @SP\n\
         AM=M-1\n\
         D=M\n\
         @R13\n\
         M=D\n\
         @{prefix}_X_NEG_{n}\n\
         D;JLT\n\
         @R14\n\
         D=M\n\
         @{prefix}_DIFF_XPOS_{n}\n\
         D;JLT\n\
         @{prefix}_SAFE_{n}\n\
         0;JMP\n\
         ({prefix}_X_NEG_{n})\n\
         @R14\n\
         D=M\n\
         @{prefix}_DIFF_XNEG_{n}\n\
         D;JGE\n\
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
         @SP\n\
         A=M\n\
         M={x}\n\
         @{prefix}_END_{n}\n\
         0;JMP\n\
         ({prefix}_DIFF_XNEG_{n})\n\
         @SP\n\
         A=M\n\
         M={y}\n\
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

pub fn translate_arithmetic(
    writer: &mut impl Write,
    operation: Operation,
    label: u16,
) -> io::Result<()> {
    // Now, every match arm cleanly passes the writer down and implicitly
    // returns the io::Result<()> from the helper function!
    match operation {
        Operation::Add => binary_op(writer, "+"),
        Operation::Sub => binary_op(writer, "-"),
        Operation::And => binary_op(writer, "&"),
        Operation::Or => binary_op(writer, "|"),
        Operation::Neg => unary_op(writer, "-"),
        Operation::Not => unary_op(writer, "!"),
        Operation::Eq => comparison_asm(writer, "EQ", "JEQ", label),
        Operation::Gt => comparison_asm(writer, "GT", "JGT", label),
        Operation::Lt => comparison_asm(writer, "LT", "JLT", label),
    }
}
