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

pub fn translate_push(segment: Segment, index: u16, filename: &str) -> String {
    match segment {
        Segment::Constant => {
            format!(
                "@{index}\n\
                D=A\n\
                {PUSH_D_TO_STACK}"
            )
        }
        Segment::Local | Segment::Argument | Segment::This | Segment::That => {
            let base = segment_base_label(segment);
            if index == 0 {
                format!(
                    "@{base}\n\
                     A=M\n\
                     D=M\n\
                     {PUSH_D_TO_STACK}"
                )
            } else {
                format!(
                    "@{base}\n\
                     D=M\n\
                     @{index}\n\
                     A=D+A\n\
                     D=M\n\
                     {PUSH_D_TO_STACK}"
                )
            }
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
        Segment::Pointer => {
            let label = match index {
                0 => "THIS",
                1 => "THAT",
                _ => unreachable!(),
            };
            format!(
                "@{label}\n\
                 D=M\n\
                 {PUSH_D_TO_STACK}"
            )
        }
        Segment::Temp => {
            let addr = 5 + index;
            format!(
                "@R{addr}\n\
                 D=M\n\
                 {PUSH_D_TO_STACK}"
            )
        }
    }
}

pub fn translate_pop(segment: Segment, index: u16, filename: &str) -> String {
    match segment {
        Segment::Constant => unreachable!(),
        Segment::Local | Segment::Argument | Segment::This | Segment::That => {
            let base = segment_base_label(segment);
            if index == 0 {
                format!(
                    "{POP_STACK_TO_D}\
                     @{base}\n\
                     A=M\n\
                     M=D\n"
                )
            } else {
                format!(
                    "@{base}\n\
                     D=M\n\
                     @{index}\n\
                     D=D+A\n\
                     @R13\n\
                     M=D\n\
                     {POP_STACK_TO_D}\
                     @R13\n\
                     A=M\n\
                     M=D\n"
                )
            }
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
        Segment::Pointer => {
            let label = match index {
                0 => "THIS",
                1 => "THAT",
                _ => unreachable!(),
            };
            format!(
                "{POP_STACK_TO_D}\
                 @{label}\n\
                 M=D\n"
            )
        }
        Segment::Temp => {
            let addr = 5 + index;
            format!(
                "{POP_STACK_TO_D}\
                 @R{addr}\n\
                 M=D\n"
            )
        }
    }
}
