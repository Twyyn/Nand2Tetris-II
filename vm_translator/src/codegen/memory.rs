use crate::parser::command::Segment;

const PUSH_D_TO_STACK: &str = "\
    @SP\n\
    A=M\n\
    M=D\n\
    @SP\n\
    M=M+1\n\
    ";

const POP_STACK_TO_D: &str = "\
    @SP\n\
    AM=M-1\n\
    D=M\n\
    ";

fn segment_base_label(segment: Segment) -> &'static str {
    match segment {
        Segment::Local => "LCL",
        Segment::Argument => "ARG",
        Segment::This => "THIS",
        Segment::That => "THAT",
        _ => unreachable!(),
    }
}

pub fn compile_push(segment: Segment, index: u16, filename: &str) -> String {
    match segment {
        Segment::Constant => {
            format!(
                "\
                @{index}\n\
                D=A\n\
                {PUSH_D_TO_STACK}\
                "
            )
        }
        Segment::Local | Segment::Argument | Segment::This | Segment::That => {
            let segment = segment_base_label(segment);
            format!(
                "\
                @{segment}\n\
                D=M\n\
                @{index}\n\
                A=D+A\n\
                D=M\n\
                {PUSH_D_TO_STACK}\
                "
            )
        }

        Segment::Static => {
            format!(
                "\
                @{filename}.{index}\n\
                D=M\n\
                {PUSH_D_TO_STACK}\
                "
            )
        }

        Segment::Temp | Segment::Pointer => {
            let base_addr = match segment {
                Segment::Temp => 5,
                Segment::Pointer => 3,
                _ => unreachable!(),
            };
            let addr = base_addr + index;
            format!(
                "\
                @R{addr}\n\
                D=M\n\
                {PUSH_D_TO_STACK}\
                "
            )
        }
    }
}

pub fn compile_pop(segment: Segment, index: u16, filename: &str) -> String {
    match segment {
        Segment::Constant => unreachable!(),
        Segment::Local | Segment::Argument | Segment::This | Segment::That => {
            let segment = segment_base_label(segment);
            format!(
                "\
                @{segment}\n\
                D=M\n\
                @{index}\n\
                D=D+A\n\
                @R13\n\
                M=D\n\
                {POP_STACK_TO_D}\
                @R13\n\
                A=M\n\
                M=D\n\
                "
            )
        }
        Segment::Static => {
            format!(
                "\
                {POP_STACK_TO_D}\
                 @{filename}.{index}\n\
                 M=D\n\
                 "
            )
        }
        Segment::Temp | Segment::Pointer => {
            let base_addr = match segment {
                Segment::Temp => 5,
                Segment::Pointer => 3,
                _ => unreachable!(),
            };
            let addr = base_addr + index;
            format!(
                "\
                {POP_STACK_TO_D}\
                @R{addr}\n\
                M=D\n\
                "
            )
        }
    }
}
