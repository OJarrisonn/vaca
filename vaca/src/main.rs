mod cli;

use std::fs;

use build::parse;
use clap::Parser;
use cli::{Cli, RunArgs, BuildArgs, Settings};
use envconfig::Envconfig;
use rustyline::config::Configurer;
use rustyline::DefaultEditor;
use speedy::{Readable, Writable};
use vaca_core::*;
use vaca_stl as stl;
use vaca_build as build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut table = SymbolTable::new();

    match std::env::var("VACA_HOME") {
        Ok(_) => {},
        Err(_) => {
            let default = if cfg!(windows) {
                std::env::var("TMP").unwrap()
            } else {
                String::from("/tmp")
            };
            
            let vaca_home = format!("{}/.vaca", homedir::get_my_home()
                .unwrap_or(Some((&default).into()))
                .unwrap_or(default.into())
                .as_path()
                .to_string_lossy());
            std::env::set_var("VACA_HOME", &vaca_home);
        },
    };

    let settings = Settings::init_from_env()?;

    table.create_scope();

    stl::load(&mut table);

    let res = match cli.command {
        Some(cmd) => match cmd {
            cli::Commands::Repl => repl(&mut table, &settings),
            cli::Commands::Run(RunArgs { file: filename }) => runner(&mut table, filename),
            cli::Commands::Build(BuildArgs { input, output}) => compiler(input, output)    
        },
        None => repl(&mut table, &settings)
    };

    table.drop_scope();

    res
}

fn repl(table: &mut SymbolTable, settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    let mut res: Result<(), Box<dyn std::error::Error>> = Ok(());

    let _ = fs::create_dir(&settings.vaca_home);

    let mut editor = DefaultEditor::new()?;
    editor.set_max_history_size(settings.repl_history_len)?;
    //editor.set_auto_add_history(true);
    let _ = editor.load_history(&format!("{}/repl_history.txt", settings.vaca_home));

    // TODO History navigation


    println!("Vaca v0.4.0 REPL");
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
                rustyline::error::ReadlineError::WindowResized => continue,
                e => { res = Err(Box::new(e)); break},
            }
        };

        if input.trim() == ";" { break; }
        if input.trim() == ";clear" { let _ = editor.clear_screen()?; continue }
        //if input.trim() == ";env" { table.env().iter().for_each(|(s, v)| println!("{s}\t=> \t{v}")); continue }
        if input.trim() == "" { continue; }

        let _ = editor.add_history_entry(&input);

        let program = match parse(input) {
            Ok(program) => program,
            Err(e) => {
                eprintln!("!>> \n{e}");
                continue;
            },
        };

        match program.eval(table) {
            Ok(v) => match v.as_ref() {    
                Value::Nil => println!(""),
                d => println!("$>> {d}")
            },
            Err(e) => eprintln!("!>> {e}"),
        }
    }

    editor.save_history(&format!("{}/repl_history.txt", &settings.vaca_home))?;

    res
}

fn compiler(input: String, output: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    if input.ends_with(".casco") {
        return Err(Box::new(GenericError(format!("{} is already a *.casco file, there's no need to compile it", input))));
    } else if !input.ends_with(".vaca") {
        return Err(Box::new(GenericError(format!("{} is not a *.vaca file", input))))
    }

    let source = fs::read_to_string(&input)?;
    let compiled = parse(format!("{{{}}}", source));

    let output = output.unwrap_or(input.replace(".vaca", ".casco"));

    match compiled {
        Ok(compiled) => Ok(compiled.write_to_file(output)?),
        Err(e) => Err(Box::new(GenericError(e))),
    }
}

fn runner(table: &mut SymbolTable, filename: String) -> Result<(), Box<dyn std::error::Error>> {
    let compiled = if filename.ends_with(".vaca") { false } 
                        else if filename.ends_with(".casco") { true } 
                        else { 
                            return Err(Box::new(GenericError(format!("The filename {} isn't a *.vaca nor *.casco file", filename))))
                        };


    let program = if compiled {
        Form::read_from_file(filename)?
    } else {
        let source = fs::read_to_string(filename)?;
    
        parse(format!("{{{}}}", source))?        
    };

    match program.eval(table) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(GenericError(e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_raw_file() {
        let mut table = SymbolTable::new();
        table.create_scope();
        stl::load(&mut table);

        let res = runner(&mut table, String::from("../tests/fib.vaca"));

        let _ = dbg!(res);

        table.drop_scope();
    }
}