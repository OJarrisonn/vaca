#[derive(Debug, Clone)]
/// Defines an external functional object defined in native Rust code
/// It's loaded from the specified library
/// 
/// Can be a function or a macro with any arity
pub struct External {
    pub lib: String,
    pub symbol: String,
    pub kind: ExternalKind,
    pub arity: u32,
    pub is_action: bool
}

#[derive(Debug, Clone)]
/// Indentifies if the external symbol is a function or a macro
pub enum ExternalKind {
    Function,
    Macro
}