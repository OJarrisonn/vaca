use std::{collections::LinkedList, rc::Rc};

use vaca_core::{register, sym, value::macros::Macro, ErrorStack, Form, Symbol, SymbolTable, Value};

pub fn load(table: &mut SymbolTable) {
    register!(table, "|>", Value::Macro(Macro::native(0, pipe_forward)));

}

fn pipe_forward(table: &mut SymbolTable, args: Vec<Form>) -> Result<Rc<Value>, ErrorStack> {
    let mut iter = args.into_iter();
    let init = iter.next();
    let init = match init {
        Some(f) => f.eval(table).map_err(|err| ErrorStack::Stream { 
            src: Some("|>".into()), 
            from: Box::new(err), 
            note: Some("Error happened whileevalutating the inital value of the piping sequence".into()) })?,
        None => return Err(ErrorStack::Top { src: Some("|>".into()), msg: "Piping must have an initial value".into() }),
    };

    iter.try_fold(init, |acc, item| {
        let target = item.eval(table)
            .map_err(|err| ErrorStack::Stream { 
                src: None, 
                from: Box::new(err), 
                note: Some("Error happened while trying to eval form to receive piping".into()) })?;
    
        match target.as_ref() {
            Value::Function(f) => f.exec(LinkedList::from([acc]), table)
                .map_err(|err| ErrorStack::Stream { src: None, from: Box::new(err), note: Some("Error happened while executing function in piping sequence".into()) }),
            v => Err(ErrorStack::Top { src: None, msg: format!("attempt to pipe in `{v}` which isn't a function. One may only pipe over functions") })
        }
    })

    
}