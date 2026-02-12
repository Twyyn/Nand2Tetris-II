use std;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let filepath = args.get(1).expect("Usage: vm_translator <file.vm>");

    vm_translator::VMTranslator::new(filepath)?.run()
}

