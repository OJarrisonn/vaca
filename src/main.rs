mod stl;
mod parser;
mod runtime;
mod cli;

#[macro_use]
mod macros;

use std::fs;

use clap::Parser;
use cli::{Cli, RunArgs, BuildArgs, Settings};
use envconfig::Envconfig;
use runtime::error::GenericError;
use rustyline::config::Configurer;
use rustyline::DefaultEditor;
use speedy::{Writable, Readable};

use crate::parser::parse;
use crate::runtime::{data::{Data, owner::Owner, symbol_table::SymbolTable}, expr::{Expr, Literal}, symbol::Symbol};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut owner = Owner::new();
    let mut table = SymbolTable::new();

    match std::env::var("VACA_HOME") {
        Ok(_) => {},
        Err(_) => {
            let vaca_home = format!("{}/.vaca", std::env::var("HOME").unwrap());
            std::env::set_var("VACA_HOME", &vaca_home);
        },
    };

    let settings = Settings::init_from_env()?;

    dbg!(&settings);

    owner.create_scope();
    table.create_scope();

    stl::load(&mut owner, &mut table);

    let res = match cli.command {
        Some(cmd) => match cmd {
            cli::Commands::Repl => repl(&mut owner, &mut table, &settings),
            cli::Commands::Run(RunArgs { file: filename }) => runner(&mut owner, &mut table, filename),
            cli::Commands::Build(BuildArgs { input, output}) => compiler(input, output)    
        },
        None => repl(&mut owner, &mut table, &settings)
    };

    table.drop_scope();
    owner.drop_scope();

    res
}

fn repl(owner: &mut Owner, table: &mut SymbolTable, settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    let mut res: Result<(), Box<dyn std::error::Error>> = Ok(());

    let _ = fs::create_dir(&settings.vaca_home);

    let mut editor = DefaultEditor::new()?;
    editor.set_max_history_size(settings.repl_history_len)?;
    editor.set_auto_add_history(true);
    let _ = editor.load_history(&format!("{}/repl_history.txt", settings.vaca_home));

    // TODO History navigation


    println!("Vaca v0.2.0 REPL");
    println!("vaca help - in the command line for the help screen");
    println!("; - to exit the repl");

    loop {
        let input = editor.readline(">>> ");

        let input = match input {
            Ok(input) => input,
            Err(e) => match e {
                rustyline::error::ReadlineError::Io(io) => { eprintln!("{}", io); continue },
                rustyline::error::ReadlineError::Eof => break,
                rustyline::error::ReadlineError::Interrupted => break,
                rustyline::error::ReadlineError::Errno(n) => { res = Err(Box::new(n)); break },
                rustyline::error::ReadlineError::WindowResized => continue,
                e => { res = Err(Box::new(e)); break},
            }
        };

        if input.trim() == ";" { break; }
        if input.trim() == ";clear" { let _ = editor.clear_screen()?; continue }
        if input.trim() == ";env" { table.env().iter().for_each(|(s, v)| println!("{s}\t=> \t{v}")); continue }
        if input.trim() == "" { continue; }

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

    editor.save_history(&format!("{}/repl_history.txt", &settings.vaca_home))?;

    res
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