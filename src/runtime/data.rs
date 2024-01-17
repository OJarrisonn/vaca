use std::rc::Weak;

use self::function::Function;

pub mod function;
pub mod owner;
pub mod symbol_table;

/// Vaca primitive data types, Data must be owned by the program Owner and it's weak references goes everywhere
/// Once the scope where the data got created is destroyed, the data is freed
#[derive(Debug)]
pub enum Data {
    Nil,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
    Array(Vec<Weak<Data>>),
    Function(Function)
}

impl Into<Vec<Weak<Data>>> for Data {
    fn into(self) -> Vec<Weak<Data>> {
        match self {
            Self::Array(a) => a,
            d => panic!("Can't turn a {} into an array like", d)
        }
    }
}

impl Data {
    pub fn as_vec(&self) -> Vec<Weak<Data>> {
        match self {
            Self::Array(a) => a.clone(),
            d => panic!("Can't turn a {} into an array like", d)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::data::{owner::Owner, Data};


    #[test]
    fn data_test() {
        let mut owner = Owner::new();
        
        owner.create_scope(); 
        let a1 = owner.insert(Data::Bool(true));
        let a2 = owner.insert(Data::Integer(91));
    let a3 = owner.insert(Data::String(String::from("Jorge")));
    let a4 = owner.insert(Data::Char('b'));
    
    let array = owner.insert(Data::Array(vec![
        a1, a2, a3, a4
        ]));
        
        dbg!(&array.upgrade());
        
        owner.drop_scope();
        
        dbg!(&owner);
        dbg!(&array.upgrade());
    }
    
    #[test]
    fn function_test() {
        let mut owner = Owner::new();
        
        owner.create_scope();
        
        
    }
}