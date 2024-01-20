use std::rc::Weak;

pub fn load(table: &mut SymbolTable) {
    register!(table, "map", function!(map, "f", "array"));
    register!(table, "reduce", function!(reduce, "f", "init", "array"));
}

fn map(table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    let f = lookup!(table, "f");
    let array = lookup!(table, "array");

    let f = match f.as_ref() {
        Data::Function(f) => f,
        _ => return Err(format!("Argument for `f` should be a function not a {f}"))
    };

    let array = array.as_vec();
    let mut res = vec![];

    for item in array.iter() {
        let mapped = f.exec(vec![item.clone()], table)?;
        let mapped = owner.allocate_return(mapped);
        res.push(mapped);
    }

    Ok(owner.allocate(Data::Array(res)))
}

fn reduce(table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    let f = lookup!(table, "f");
    let init = table.lookup(&symbol!("init"))?;
    let array = lookup!(table, "array");

    let f = match f.as_ref() {
        Data::Function(f) => f,
        _ => return Err(format!("Argument for `f` should be a function not a {f}"))
    };

    let array = array.as_vec();
    let mut acc = init;

    for item in array.iter() {
        acc = f.exec(vec![acc, item.clone()], table)?;
    }

    Ok(owner.relocate(extract!(acc)))
}