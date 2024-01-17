mod stl;
mod parser;
mod runtime;
mod cli;

#[macro_use]
mod macros;

use std::fs;
use std::io::Write;

use clap::Parser;
use cli::{Cli, RunArgs, BuildArgs};
use runtime::error::GenericError;
use speedy::{Writable, Readable};

use crate::parser::parse;
use crate::runtime::{data::{Data, owner::Owner, symbol_table::SymbolTable}, expr::{Expr, Literal}, symbol::Symbol};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut owner = Owner::new();
    let mut table = SymbolTable::new();

    owner.create_scope();
    table.create_scope();

    stl::load(&mut owner, &mut table);

    let res = match cli.command {
        cli::Commands::Repl => repl(&mut owner, &mut table),
        cli::Commands::Run(RunArgs { file: filename }) => runner(&mut owner, &mut table, filename),
        cli::Commands::Build(BuildArgs { input, output}) => compiler(input, output)
    };

    table.drop_scope();
    owner.drop_scope();

    res
}

fn repl(owner: &mut Owner, table: &mut SymbolTable) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut input = String::new();

        print!(">>> ");
        let _ = std::io::stdout().flush();
        let _ = std::io::stdin().read_line(&mut input);

        if input.trim() == "." { break; }

        let program = match parse(input) {
            Ok(program) => program,
            Err(e) => {
                eprintln!("!>> \n{e}");
                continue;
            },
        };

        match program.eval(owner, table) {
            Ok(v) => match v.upgrade() { 
                Some(v) => match v.as_ref() {    
                    Data::Nil => println!(""),
                    d => println!("$>> {d}")
                },
                None => return Err(Box::new(GenericError(format!("A form returned a value that got freed"))))
            },
            Err(e) => eprintln!("!>> {e}"),
        }

    }

    Ok(())
}

fn compiler(input: String, output: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    if input.ends_with(".leite") {
        return Err(Box::new(GenericError(format!("{} is already a *.leite file, there's no need to compile it", input))));
    } else if !input.ends_with(".vaca") {
        return Err(Box::new(GenericError(format!("{} is not a *.vaca file", input))))
    }

    let source = fs::read_to_string(&input)?;
    let compiled = parse(format!("{{{}}}", source));

    let output = output.unwrap_or(input.replace(".vaca", ".leite"));

    match compiled {
        Ok(compiled) => Ok(compiled.write_to_file(output)?),
        Err(e) => Err(Box::new(GenericError(e))),
    }
}

fn runner(owner: &mut Owner, table: &mut SymbolTable, filename: String) -> Result<(), Box<dyn std::error::Error>> {
    let compiled = if filename.ends_with(".vaca") { false } 
                        else if filename.ends_with(".leite") { true } 
                        else { 
                            return Err(Box::new(GenericError(format!("The filename {} isn't a *.vaca nor *.leite file", filename))))
                        };


    let program = if compiled {
        Expr::read_from_file(filename)?
    } else {
        let source = fs::read_to_string(filename)?;
    
        parse(format!("{{{}}}", source))?        
    };

    match program.eval(owner, table) {
        Ok(_data) => Ok(()),
        Err(e) => Err(Box::new(GenericError(e))),
    }
}