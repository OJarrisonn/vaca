use std::{collections::LinkedList, rc::Rc};

use speedy::{Readable, Writable};

use crate::{ErrorStack, Form, SymbolTable, Value};

#[derive(Debug, Clone, Writable, Readable)]
pub struct Call {
    pub callable: Box<Form>,
    pub arguments: Vec<Form>,
}

impl Call {
    pub fn exec(&self, table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
        match self.callable.eval(table).map_err(|err| ErrorStack::Stream { src: None, from: Box::new(err), note: Some("Error happened while trying to evaluate callable element of the current call".into()) })?.as_ref() {
            Value::Function(f) => {
                let args = Form::Array(self.arguments.clone()).eval(table);

                match args {
                    Err(err) => Err(ErrorStack::Stream { src: Some(self.callable.to_string()), from: Box::new(err), note: Some("Error happened while evaluating an argument of the current call".into()) }),
                    Ok(args) => f.exec(args.to_array(), table)
                        .map_err(|err| ErrorStack::Stream { src: Some(self.callable.to_string()), from: Box::new(err), note: None })
                }
            },
            Value::Macro(m) => {
                let mut forms = LinkedList::new();
                for arg in self.arguments.iter() {
                    forms.push_back(arg.clone())
                }

                m.exec(table, forms).map_err(|err| ErrorStack::Stream { src: Some(self.callable.to_string()), from: Box::new(err), note: None })
            },
            Value::LazyCall(l) => l.exec(table).map_err(|err| ErrorStack::Stream { src: None, from: Box::new(err), note: Some("Error happened while resolving lazy call in the callable element of the current call".into()) }),
            _ => unreachable!()
        }
    }
}