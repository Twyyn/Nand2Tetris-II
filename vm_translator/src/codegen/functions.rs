use crate::Write;
use crate::codegen::Result;
use crate::parser::command::Function;

const RETURN_ASM: &str = "\
    // return\n\
    @LCL\n\
    D=M\n\
    @R13\n\
    M=D\n\
    @5\n\
    A=D-A\n\
    D=M\n\
    @R14\n\
    M=D\n\
    @SP\n\
    AM=M-1\n\
    D=M\n\
    @ARG\n\
    A=M\n\
    M=D\n\
    @ARG\n\
    D=M+1\n\
    @SP\n\
    M=D\n\
    @R13\n\
    AM=M-1\n\
    D=M\n\
    @THAT\n\
    M=D\n\
    @R13\n\
    AM=M-1\n\
    D=M\n\
    @THIS\n\
    M=D\n\
    @R13\n\
    AM=M-1\n\
    D=M\n\
    @ARG\n\
    M=D\n\
    @R13\n\
    AM=M-1\n\
    D=M\n\
    @LCL\n\
    M=D\n\
    @R14\n\
    A=M\n\
    0;JMP\n\
    ";

pub fn translate_function(writer: &mut impl Write, function: Function, label: u16) -> Result<()> {
    match function {
        Function::Declare { name, var_count } => {
            write!(writer, "({name})\n")?;

            if var_count <= 8 {
                for _ in 0..var_count {
                    write!(writer, "@SP\nA=M\nM=0\n@SP\nM=M+1\n")?;
                }
            } else {
                write!(
                    writer,
                    "\
                    @{var_count}\n\
                    D=A\n\
                    @R13\n\
                    M=D\n\
                    (INIT_LOCALS_{label})\n\
                    @R13\n\
                    D=M\n\
                    @END_INIT_{label}\n\
                    D;JEQ\n\
                    @SP\n\
                    A=M\n\
                    M=0\n\
                    @SP\n\
                    M=M+1\n\
                    @R13\n\
                    M=M-1\n\
                    @INIT_LOCALS_{label}\n\
                    0;JMP\n\
                    (END_INIT_{label})\n\
                    "
                )?;
            }
            Ok(())
        }

        Function::Call { name, arg_count } => {
            let return_label = format!("{name}$ret.{label}");
            write!(
                writer,
                "\
                // call {name} {arg_count}\n\
                 @{return_label}\n\
                 D=A\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n\
                 @LCL\n\
                 D=M\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n\
                 @ARG\n\
                 D=M\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n\
                 @THIS\n\
                 D=M\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n\
                 @THAT\n\
                 D=M\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n\
                 @SP\n\
                 D=M\n\
                 @5\n\
                 D=D-A\n\
                 @{arg_count}\n\
                 D=D-A\n\
                 @ARG\n\
                 M=D\n\
                 @SP\n\
                 D=M\n\
                 @LCL\n\
                 M=D\n\
                 @{name}\n\
                 0;JMP\n\
                 ({return_label})\n\
                 "
            )
        }

        Function::Return => write!(writer, "{}", RETURN_ASM),
    }
}
