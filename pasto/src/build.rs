use vaca_core::build::program::Program as BuildProgram;

use crate::build::table::TrackTable;
use crate::BuildResult;

mod table;
mod symbol_validation;

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