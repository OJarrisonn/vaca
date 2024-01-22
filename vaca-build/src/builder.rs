use vaca_core::ErrorStack;
use speedy::Writable;

use crate::parse_program;



pub fn build(input: String, output: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    if input.ends_with(".casco") {
        return Err(Box::new(ErrorStack::from(format!("{} is already a *.casco file, there's no need to compile it", input))));
    } else if !input.ends_with(".vaca") {
        return Err(Box::new(ErrorStack::from(format!("{} is not a *.vaca file", input))))
    }

    let source = std::fs::read_to_string(&input)?;
    let compiled = parse_program(source);

    let output = output.unwrap_or(input.replace(".vaca", ".casco"));

    match compiled {
        Ok(compiled) => Ok(compiled.write_to_file(output)?),
        Err(e) => Err(Box::new(ErrorStack::Stream { src: None, from: Box::new(e), note: Some("Error from parsing step".into()) })),
    }
}