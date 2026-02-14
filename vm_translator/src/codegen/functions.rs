use crate::parser::command::FunctionCommand;

pub fn compile_function(
    function_command: FunctionCommand,
    filename: &str,
    label_count: u16,
) -> String {
    match function_command {
        FunctionCommand::Function {
            function_name,
            local_count,
        } => {
            format!(
                "\
                ({filename}.{function_name})\n\
                @SP\n\
                D=M\n\
                @LCL\n\
                M=D\n\
                @i\n\
                M=0\n\
                (PUSH_VAR_LOOP_START{label_count})\n\
                @{local_count}\n\
                D=A\n\
                @i\n\
                D=D-M\n\
                @PUSH_VAR_LOOP_END_{label_count}\n\
                D;JEQ\n\
                @LCL\n\
                D=M\n\
                @i\n\
                A=D+M\n\
                M=0\n\
                @i\n\
                M=M+1\n\
                (PUSH_VAR_LOOP_START{label_count})\n\
                0;JMP\n\
                @PUSH_VAR_LOOP_END_{label_count}\n\
                @i\n\
                D=M\n\
                @SP\n\
                M=D+M\n\
                "
            )
        }
        FunctionCommand::Call {
            function_name,
            arg_count,
        } => {
            format!(
                "@Foo.bar&ret.1
D=A

@SP
A=M
M=D

@SP
M=M+1

@LCL
D=M

@SP
A=M
M=D
@SP
M=M+1

@ARG
D=M

@SP
A=M
M=D
@SP
M=M+1

@THIS
D=M

@SP
A=M
M=D

@SP
M=M+1

@THAT
D=M

@SP
A=M
M=D

@SP
M=M+1

@SP
D=M
@5
D=D-A

@2
D=D-A

@ARG
M=D

@SP
D=M

@LCL
M=D

@Foo.bar
0;JMP

(Foo.bar&ret.1)



"
            )
        }
        FunctionCommand::Return => {
            format!(
                "@LCL
D=M
@R13
M=D


@5
A=D-A
D=M
@R14
M=D


@SP
AM=M-1
D=M
@ARG
A=M
M=D


@ARG
D=M+1
@SP
M=D


@R13
AM=M-1
D=M
@THAT
M=D


@R13
AM=M-1
D=M
@THIS
M=D


@R13
AM=M-1
D=M
@ARG
M=D

@R13
AM=M-1
D=M
@LCL
M=D

@R14
A=M
0;JMP"
            )
        }
    }
}
