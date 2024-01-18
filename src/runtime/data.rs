use std::{rc::Weak, iter::zip};

use self::function::Function;

pub mod function;
pub mod owner;
pub mod symbol_table;

/// Vaca primitive data types, Data must be owned by the program Owner and it's weak references goes everywhere
/// Once the scope where the data got created is destroyed, the data is freed
#[derive(Debug, Clone)]
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

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0.partial_cmp(r0),
            (Self::Integer(l0), Self::Float(r0)) => (*l0 as f64).partial_cmp(r0),
            (Self::Float(l0), Self::Float(r0)) => l0.partial_cmp(r0),
            (Self::Float(l0), Self::Integer(r0)) => l0.partial_cmp(&(*r0 as f64)),
            (Self::Char(l0), Self::Char(r0)) => l0.partial_cmp(r0),
            (Self::String(l0), Self::String(r0)) => l0.partial_cmp(r0),
            _ => None
        }
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Float(r0)) => *l0 as f64 == *r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Float(l0), Self::Integer(r0)) => *l0 == *r0 as f64,
            (Self::Char(l0), Self::Char(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Array(l0), Self::Array(r0)) => {
                if l0.len() != r0.len() {
                    false
                } else {
                    zip(l0, r0).all(|(l, r)| l.upgrade() == r.upgrade())
                }
            },
            (Self::Function(_), Self::Function(_)) => false,
            _ => false//core::mem::discriminant(self) == core::mem::discriminant(other),
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
        let a1 = owner.allocate(Data::Bool(true));
        let a2 = owner.allocate(Data::Integer(91));
    let a3 = owner.allocate(Data::String(String::from("Jorge")));
    let a4 = owner.allocate(Data::Char('b'));
    
    let array = owner.allocate(Data::Array(vec![
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