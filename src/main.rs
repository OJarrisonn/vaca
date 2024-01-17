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

use crate::parser::parse;
use crate::runtime::{data::{Data, owner::Owner, symbol_table::SymbolTable}, expr::{Expr, Literal}, symbol::Symbol};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut owner = Owner::new();
    let mut table = SymbolTable::new();

    owner.create_scope();
    table.create_scope();

    stl::load(&mut owner, &mut table);

    let res = match &cli.command {
        cli::Commands::Repl => repl(&mut owner, &mut table),
        cli::Commands::Run(RunArgs { file: filename }) => runner(&mut owner, &mut table, filename),
        cli::Commands::Build(BuildArgs { input, output}) => compiler(&mut owner, &mut table, input, output)
    };

    table.drop_scope();
    owner.drop_scope();

    res
}

fn repl(owner: &mut Owner, table: &mut SymbolTable) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut input = String::new();

        print!("$>> ");
        let _ = std::io::stdout().flush();
        let _ = std::io::stdin().read_line(&mut input);

        if input.trim() == "." { break; }

        let program = parse(input).unwrap();
        let res = program.eval(owner, table);

        dbg!(res.unwrap().upgrade());
        println!("");
    }

    Ok(())
}

fn compiler(owner: &mut Owner, table: &mut SymbolTable, input: &str, output: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    if input.ends_with(".leite") {
        return Err(Box::new(GenericError(format!("{} is already a *.leite file, there's no need to compile it", input))));
    } else if !input.ends_with(".vaca") {
        return Err(Box::new(GenericError(format!("{} is not a *.vaca file", input))))
    }

    let source = fs::read_to_string(input)?;
    let compiled = parse(source);

    match compiled {
        Ok(compiled) => todo!(),
        Err(e) => Err(Box::new(GenericError(e))),
    }
}

fn runner(owner: &mut Owner, table: &mut SymbolTable, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let compile = if filename.ends_with(".vaca") { false } 
                        else if filename.ends_with(".leite") { true } 
                        else { 
                            return Err(Box::new(GenericError(format!("The filename {} isn't a *.vaca nor *.leite file", filename))))
                        };


    if compile {
        return Err(Box::new(GenericError("WIP: Running compiled files is not implemented".to_string())));
    } else {
        let source = fs::read_to_string(filename)?;
    
        let ast = parse(format!("{{{}}}", source));

        match ast {
            Ok(program) => match program.eval(owner, table) {
                Ok(_data) => Ok(()),
                Err(e) => Err(Box::new(GenericError(e))),
            },
            Err(e) => Err(Box::new(GenericError(e))),
        }
    }
}