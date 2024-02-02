use std::error::Error;

use vaca_core::build::error::BuildErrorStack;
use vaca_core::build::program::Program as BuildProgram;
use vaca_core::run::program::Program as RunProgram;

use crate::build::table::TrackTable;
use crate::BuildResult;

mod table;
mod symbol_validation;

/// Takes a program structure and sets it runtime-ready by creating a SymbolTable and checking for undefined symbols
pub fn build_program(program: BuildProgram) -> BuildResult<RunProgram> {
    let forms = program.forms();

    let mut track = TrackTable::new();
    track.create_scope();
    populate_track(&mut track);

    let mut sval_errs = forms.iter()
        .map(|form| symbol_validation::validate_form(&mut track, form))
        .filter(Result::is_err)
        .map(Result::unwrap_err)
        .collect::<Vec<BuildErrorStack>>();

    if sval_errs.is_empty() {
        Ok(RunProgram::from(program))
    } else if sval_errs.len() == 1 {
        Err(sval_errs.pop().unwrap())
    } else {
        Err(BuildErrorStack::MultiStream { from: sval_errs.into_iter().map(|err| Box::new(err) as Box<dyn Error>).collect(), src: "program".into(), note: None })
    }
}

pub fn validate_program(program: &BuildProgram) -> BuildResult<()> {
    symbol_validation::validate_program(program)
}

/// Temporary function to load the stl symbols into the TrackTable
fn populate_track(track: &mut TrackTable) {
    let stl = ["+", "-", "*", "/", "^", "map", "reduce", "fold", "scan", "filter", "append", "prepend", "pop-back", "pop-front", "print", "println", "readln", "format"];

    for symbol in stl {
        let _ = track.assign(&symbol.into());
    }
}