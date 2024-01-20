use vaca_build::parse;
use vaca_core::{SymbolTable, Value};
use vaca_stl as stl;

use rustyline::config::Configurer;
use rustyline::DefaultEditor;

use crate::settings::Settings;

/// The Vaca Repl itself, contains it's own SymbolTable and set of settings
/// 
/// The Repl is an interactive environment that reads the user input, evaluate it as a Vaca program, prints the resulting value and loops agains
pub struct Repl {
    settings: Settings,
    table: SymbolTable
}

// TODO multiline forms

impl Repl {
    /// Creates a new Repl with the given Settings
    pub fn new(settings: Settings) -> Self {
        let mut repl = Self {
            settings,
            table: SymbolTable::new()
        };

        repl.table.create_scope();
        
        repl
    } 

    /// Load the stl libraries into the repl
    pub fn load_stl(self) -> Self {
        Self { table: stl::load(self.table), ..self }
    }

    /// Starts the repl. Returns Ok(()) when finished or an Err(...) when some unrecoverable error happens durring the Repl
    pub fn repl(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut res: Result<(), Box<dyn std::error::Error>> = Ok(());

        let _ = std::fs::create_dir(&self.settings.vaca_home);

        let mut editor = DefaultEditor::new()?;
        editor.set_max_history_size(self.settings.repl_history_len)?;
        let _ = editor.load_history(&format!("{}/repl_history.txt", self.settings.vaca_home));
        
        println!("Vaca {} REPL", self.settings.version);
        println!("vaca help - in the command line for the help screen");
        println!("; - to exit the repl");
        
        self.table.create_scope(); // The loaded STL will not be overwritten

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

            match program.eval(&mut self.table) {
                Ok(v) => match v.as_ref() {    
                    Value::Nil => println!(""),
                    d => println!("$>> {d}")
                },
                Err(e) => eprintln!("!>> {e}"),
            }
        }

        editor.save_history(&format!("{}/repl_history.txt", &self.settings.vaca_home))?;
        
        self.table.drop_scope();
        self.table.drop_scope(); // Drops the scope created in Repl::new

        res
    }
}
