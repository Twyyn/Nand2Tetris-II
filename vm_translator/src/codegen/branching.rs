use crate::parser::command::BranchCommand;

pub fn compile_branch(command: BranchCommand) -> String {
    match command {
        BranchCommand::Label { label } => {
            format!("({label})\n")
        }
        BranchCommand::Goto { label } => {
            format!(
                "\
                @{label}\n\
                0;JMP\n\
                "
            )
        }
        BranchCommand::IfGoto { label } => {
            format!(
                "\
                @SP\n\
                AM=M-1\n\
                D=M\n\
                @{label}\n\
                D;JNE\n\
                "
            )
        }
    }
}
