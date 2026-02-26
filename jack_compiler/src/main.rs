use jack_compiler::lexer::Lexer;

fn main() {
    let source = " \"this is a string ";
    let lexer = Lexer::new(source);

    for token in lexer.tokenize() {
        println!("{}", token.as_xml(source));
    }
}
