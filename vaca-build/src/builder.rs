use vaca_core::GenericError;
use speedy::Writable;

use crate::parse;



pub fn build(input: String, output: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    if input.ends_with(".casco") {
        return Err(Box::new(GenericError(format!("{} is already a *.casco file, there's no need to compile it", input))));
    } else if !input.ends_with(".vaca") {
        return Err(Box::new(GenericError(format!("{} is not a *.vaca file", input))))
    }

    let source = std::fs::read_to_string(&input)?;
    let compiled = parse(format!("{{{}}}", source));

    let output = output.unwrap_or(input.replace(".vaca", ".casco"));

    match compiled {
        Ok(compiled) => Ok(compiled.write_to_file(output)?),
        Err(e) => Err(Box::new(GenericError(e))),
    }
}