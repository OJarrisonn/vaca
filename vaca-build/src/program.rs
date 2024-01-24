use vaca_core::Symbol;

pub struct Program {
    imports: Vec<Import>
}

pub struct Import {
    namespace: String,
    path: String,
    symbols: Vec<Symbol>
}