use crate::parser::command::Br;

pub fn branch(instruction: &Br) -> String {
    match instruction {
        Br::Label { label } => {
            format!("(label)\n")
        }
        Br::Goto { label } => todo!(),
        Br::IfGoto { label } => todo!(),
    }
}
