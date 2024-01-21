use vaca_core::*;

mod io;
mod logic;
mod array;
mod math;

// TODO: Learn about as_ref for *const and refactor the code
pub fn load(mut table: SymbolTable) -> SymbolTable {
    math::load(&mut table);
    io::load(&mut table);
    logic::load(&mut table);
    array::load(&mut table);
    table
}

pub fn create_table() -> SymbolTable {
    let mut table = SymbolTable::new();
    table.create_scope();
    load(table)
}

#[cfg(test)]
mod tests {
    
}
