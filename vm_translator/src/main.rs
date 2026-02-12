fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = std::env::args()
        .nth(1)
        .ok_or("Usage: vm_translator <file.vm>")?;

    // vm_translator::VMTranslator::new(&filepath)?.run()?;
    println!("{:?}", vm_translator::VMTranslator::new(&filepath)?);
    Ok(())
}
