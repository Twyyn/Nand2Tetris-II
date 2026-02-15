use crate::parser::command::Branch;

pub fn translate_branch(command: Branch, current_function: &str) -> String {
    match command {
        Branch::Label { label } => {
            format!("({current_function}${label})\n")
        }
        Branch::Goto { label } => {
            format!(
                "@{current_function}${label}\n\
                 0;JMP\n"
            )
        }
        Branch::IfGoto { label } => {
            format!(
                "@SP\n\
                 AM=M-1\n\
                 D=M\n\
                 @{current_function}${label}\n\
                 D;JNE\n"
            )
        }
    }
}
