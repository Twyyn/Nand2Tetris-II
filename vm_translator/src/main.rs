use std::env;
use vm_translator::vm_translator::VMTranslator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: vm_translator <file.vm>");

    match VMTranslator::new(filename) {
        Ok(translator) => translator.run(),
        Err(e) => eprintln!("{e}"),
    }
}
