use crate::{
    codegen::{UniqueLabel, functions::translate_function},
    parser::command::Function,
};

pub fn bootstrap(filename: &str, label: UniqueLabel) -> String {
    let call_sys_init = translate_function(
        Function::Call {
            name: "Sys.init".to_string(),
            arg_count: 0,
        },
        label,
    );

    format!(
        "// Bootstrap\n\
        @256\n\
        D=A\n\
        @SP\n\
        M=D\n\
        {call_sys_init}"
    )
}
