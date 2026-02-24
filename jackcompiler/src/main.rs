use jackcompiler::{JackCompiler, lexer::Lexer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source_path = std::env::args()
        .nth(1)
        .ok_or("Usage: JackCompiler <file.jack | directory>")?;

    let compiler = JackCompiler::new(&source_path)?;
    let source_files = compiler.source;
    let mut source = String::new();
    for file in source_files {
        source.push_str(&file.contents);
    }
    let mut lexer = Lexer::new(&source);
    println!("{:#?}", lexer.scan());

    Ok(())
}
