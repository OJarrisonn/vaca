use vaca_core::*;

mod io;
mod logic;
mod array;
mod math;

pub fn load(table: &mut SymbolTable) {
    math::load(table);
    io::load(table);
    logic::load(table);
    array::load(table);
}

#[cfg(test)]
mod tests {
    
}
