use crate::Write;
use crate::codegen::Result;
use crate::parser::command::Segment;

const PUSH_D: &str = "\
    @SP\n\
    A=M\n\
    M=D\n\
    @SP\n\
    M=M+1\n";

const POP_D: &str = "\
    @SP\n\
    AM=M-1\n\
    D=M\n";

fn base_label(segment: Segment) -> &'static str {
    match segment {
        Segment::Local => "LCL",
        Segment::Argument => "ARG",
        Segment::This => "THIS",
        Segment::That => "THAT",
        _ => unreachable!(),
    }
}

/// Resolves a direct-mapped segment to its assembly address symbol.
fn direct_address(segment: Segment, index: u16, filename: &str) -> String {
    match segment {
        Segment::Static => format!("{filename}.{index}"),
        Segment::Pointer => (if index == 0 { "THIS" } else { "THAT" }).to_string(),
        Segment::Temp => format!("R{}", 5 + index),
        _ => unreachable!(),
    }
}

pub fn translate_push(
    writer: &mut impl Write,
    segment: Segment,
    index: u16,
    filename: &str,
) -> Result<()> {
    match segment {
        Segment::Constant => {
            write!(writer, "@{index}\nD=A\n{PUSH_D}")
        }
        Segment::Local | Segment::Argument | Segment::This | Segment::That => {
            let base = base_label(segment);
            if index == 0 {
                write!(writer, "@{index}\nD=A\n{PUSH_D}")
            } else {
                write!(writer, "@{base}\nD=M\n@{index}\nA=D+A\nD=M\n{PUSH_D}")
            }
        }
        Segment::Static | Segment::Pointer | Segment::Temp => {
            let addr = direct_address(segment, index, filename);
            write!(writer, "@{addr}\nD=M\n{PUSH_D}")
        }
    }
}

pub fn translate_pop(
    writer: &mut impl Write,
    segment: Segment,
    index: u16,
    filename: &str,
) -> Result<()> {
    match segment {
        Segment::Constant => unreachable!(),
        Segment::Local | Segment::Argument | Segment::This | Segment::That => {
            let base = base_label(segment);
            if index == 0 {
                write!(writer, "{POP_D}@{base}\nA=M\nM=D\n")
            } else {
                write!(
                    writer,
                    "@{base}\nD=M\n@{index}\nD=D+A\n@R13\nM=D\n\
                     {POP_D}@R13\nA=M\nM=D\n"
                )
            }
        }
        Segment::Static | Segment::Pointer | Segment::Temp => {
            let addr = direct_address(segment, index, filename);
            write!(writer, "{POP_D}@{addr}\nM=D\n")
        }
    }
}
