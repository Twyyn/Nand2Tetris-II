use jackcompiler::lexer::Lexer;

fn main() {
    println!("Hello, world!");
    let s = " do Math.sqrt() \n if something";
    let mut lexer = Lexer::new(s);
    println!("{:?}", lexer.scan());
}
