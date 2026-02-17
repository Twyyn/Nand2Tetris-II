use crate::Write;
use crate::codegen::Result;
use crate::parser::command::Branch;

pub fn translate_branch(
    writer: &mut impl Write,
    command: Branch,
    current_function: &str,
) -> Result<()> {
    match command {
        Branch::Label { label } => {
            write!(writer, "({current_function}${label})\n")
        }
        Branch::Goto { label } => {
            write!(
                writer,
                "@{current_function}${label}\n\
                 0;JMP\n"
            )
        }
        Branch::IfGoto { label } => {
            write!(
                writer,
                "@SP\n\
                 AM=M-1\n\
                 D=M\n\
                 @{current_function}${label}\n\
                 D;JNE\n"
            )
        }
    }
}
