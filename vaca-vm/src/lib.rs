use vaca_core::*;
use vaca_build as build;
use vaca_stl as stl;

use speedy::Readable;

pub fn run(filename: String) -> Result<(), Box<dyn std::error::Error>> {
    let compiled = if filename.ends_with(".vaca") { false } 
                        else if filename.ends_with(".casco") { true } 
                        else { 
                            return Err(Box::new(ErrorStack::from(format!("The filename {} isn't a *.vaca nor *.casco file", filename))))
                        };
    
    let mut table = stl::create_table();
    table.create_scope();
    

    let program = if compiled {
        Form::read_from_file(filename)?
    } else {
        let source = std::fs::read_to_string(filename)?;
    
        build::parse_program(source)?        
    };

    let res = match program.eval(&mut table) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(ErrorStack::Stream{src: None, from: Box::new(e), note: Some("Error during evaluation of the source program".into())})),
    };

    table.drop_scope();
    table.drop_scope(); // Drops the scope created by STL

    Ok(res?)
}

#[cfg(test)]
mod tests {
    
}
