fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = std::env::args()
        .nth(1)
        .ok_or("Usage: vm_translator <file.vm>")?;

    vm_translator::VMTranslator::new(&input_path)?.run()?;
    //println!("{:?}", vm_translator::VMTranslator::new(&input_path)?);
    Ok(())
}
   