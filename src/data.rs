use std::rc::Rc;

#[derive(Debug)]
pub enum Data {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
    Array(Vec<Rc<Data>>)
}

#[derive(Debug)]
pub struct DataScope(Vec<Rc<Data>>);

#[derive(Debug)]
pub struct Owner {
    owned: Vec<DataScope>
}

impl DataScope {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl Owner {
    pub fn new() -> Self {
        Self {
            owned: vec![]
        }
    }

    pub fn create_scope(&mut self) {
        self.owned.push(DataScope::new());
    }

    pub fn drop_scope(&mut self) {
        self.owned.pop();
    }

    pub fn insert(&mut self, data: Data) -> Rc<Data> {
        { self.owned.last_mut().unwrap().0.push(Rc::new(data)); }

        Rc::clone(self.owned.last().unwrap().0.last().unwrap())
    }
}

#[test]
fn data_test() {
    let mut owner = Owner::new();

    owner.create_scope(); 
    let data = owner.insert(Data::Bool(true));
    let array = owner.insert(Data::Array(vec![
        Rc::new(Data::Integer(7)),
        Rc::new(Data::Integer(8)),
        Rc::new(Data::Float(9.8)),
        Rc::new(Data::Char('x')),
    ]));

    owner.drop_scope();

    dbg!(owner);
    dbg!(Rc::strong_count(&data));
    dbg!(array);
}