use vaca_core::{build::{form::{call::Call, function::Function as FunctionForm, macros::Macro as MacroForm, Expr, Form}, symbol::Symbol}, run::{error::RunErrorStack, result::RunResult, value::{function::Function as FunctionValue, macros::Macro as MacroValue, Value}, valueref::ValueRef}};

use self::{execute::{execute_function, execute_macro}, table::SymbolTableStack};

mod execute;
mod table;
mod native;

/// Takes a form a stack table and executes the form returning the resulting value
pub fn run_form(table: &mut SymbolTableStack, form: Form) -> RunResult<ValueRef> {
    match form.into_expr() {
        Expr::Symbol(s) => table.lookup(&s),
        Expr::AssignmentList(list) => run_assignment_list(table, list),
        Expr::Scope(scope) => run_scope(table, scope),
        Expr::Function(func) => run_function(table, func),
        Expr::Macro(mac) => run_macro(table, mac),
        Expr::Call(call) => run_call(table, call),
        Expr::Array(array) => run_array(table, array).map(|ok| ValueRef::new(Value::Array(ok.into_iter().collect()))),
        Expr::Capture(cap) => Ok(cap),
        lit => Ok(from_literal(lit))
    }
}

pub fn run_array(table: &mut SymbolTableStack, array: Vec<Form>) -> RunResult<Vec<ValueRef>> {
    array.into_iter()
        .map(|form| run_form(table, form))
        .collect::<Result<Vec<ValueRef>, RunErrorStack>>()
}

/// Takes an expression which is any literal (Nil, Integer, Float, String, Bool or Atom) and produces the corresponding value
fn from_literal(expr: Expr) -> ValueRef {
    match expr {
        Expr::Nil => ValueRef::new(Value::Nil),
        Expr::Integer(i) => ValueRef::new(Value::Integer(i)),
        Expr::Float(f) => ValueRef::new(Value::Float(f)),
        Expr::String(s) => ValueRef::new(Value::String(s)),
        Expr::Bool(b) => ValueRef::new(Value::Bool(b)),
        Expr::Atom(a) => ValueRef::new(Value::Atom(a)),
        _ => unreachable!()
    }
}

/// Takes a `SymbolTableStack` and a list of assignments and populate the current scope with the assignments
fn run_assignment_list(table: &mut SymbolTableStack, list: Vec<(Symbol, Form)>) -> RunResult<ValueRef> {
    let errs = list.into_iter().map(|(s, f)| {
        let res = run_form(table, f)?;
        table.assign(s, res, false) // TODO: check if the passed form is an action
    }).filter(Result::is_err)
    .map(Result::unwrap_err)
    .collect::<Vec<RunErrorStack>>();

    RunErrorStack::into_result(errs, ValueRef::new(Value::Nil))
}

/// Takes a form which is a scope, creates a new scope in the table runs the forms inside the scope, drops the scope and finally returns the result of the last form
fn run_scope(table: &mut SymbolTableStack, mut scope: Vec<Form>) -> RunResult<ValueRef> {
    table.create_scope();
    let last = scope.pop(); // The form whose value gonna be returned

    let res = match last {
        Some(last) => {
            for form in scope.into_iter() {
                // Executes each form in the scope and drops it's resulting value
                drop(run_form(table, form)?);
            }

            run_form(table, last)
        },
        None => Ok(ValueRef::new(Value::Nil)),
    };

    table.drop_scope();
    res
}

/// Takes a function form, does the needed captures and produces a function value
fn run_function(table: &mut SymbolTableStack, FunctionForm {parameters, body}: FunctionForm) -> RunResult<ValueRef> {
    // Cria uma função realizando as capturas necessárias
    let func = FunctionValue::new(parameters, realize_captures(table, *body));
    
    Ok(ValueRef::new(Value::Function(func)))
}

/// Takes a function form, does the needed captures and produces a function value
fn run_macro(table: &mut SymbolTableStack, MacroForm {parameters, body}: MacroForm) -> RunResult<ValueRef> {
    // Cria uma função realizando as capturas necessárias
    let mac = MacroValue::new(parameters, realize_captures(table, *body));
    
    Ok(ValueRef::new(Value::Macro(mac)))
}

fn realize_captures(table: &mut SymbolTableStack, Form { expr, span }: Form) -> Form {
    let expr = match expr {
        // Symbols are captured to their known value at this point
        Expr::Symbol(ref symbol) => match table.lookup(symbol) {
            Ok(value) => Expr::Capture(value),
            Err(_) => return Form { expr, span },
        },
        Expr::AssignmentList(list) => 
            Expr::AssignmentList(list.into_iter()
                .map(|(symbol, form)| (symbol, realize_captures(table, form)))
                .collect()),

        Expr::Scope(scope) => 
            Expr::Scope(scope.into_iter().map(|form| realize_captures(table, form)).collect()),

        Expr::Function(FunctionForm { parameters, body }) => 
            Expr::Function(FunctionForm { parameters, body: Box::new(realize_captures(table, *body)) }),

        Expr::Macro(MacroForm { parameters, body }) => 
            Expr::Macro(MacroForm { parameters, body: Box::new(realize_captures(table, *body)) }),

        Expr::Call(Call { callable, arguments }) => 
            Expr::Call(Call { 
                callable: Box::new(realize_captures(table, *callable)), 
                arguments: arguments.into_iter()
                    .map(|form| realize_captures(table, form)).collect() 
            }),

        Expr::Array(array) => Expr::Array(array.into_iter().map(|form| realize_captures(table, form)).collect()),
        _ => return Form { expr, span }
    };

    Form::new(span, expr)
}

/// Core function which takes a call form and evaluates it linearly. No parallelism is applied to it's parameters
pub fn run_call(table: &mut SymbolTableStack, Call { callable, arguments }: Call) -> RunResult<ValueRef> {
    let callable = *callable;
    let argc = arguments.len();

    // We try to evaluate the callable part of the function and raise any possible error, getting only the value of the callable
    let callable_value = match callable.expr {
        Expr::Symbol(symbol) => table.lookup(&symbol),
        Expr::Scope(scope) => run_scope(table, scope),
        Expr::Function(func) => run_function(table, func),
        Expr::Macro(mac) => run_macro(table, mac),
        Expr::Call(call) => run_call(table, call),
        Expr::Capture(cap) => Ok(cap),
        _ => if argc > 0 {
            return Err(RunErrorStack::Top { src: Some(callable.span.to_string()), msg: format!("Passed arguments to non-functional form") })
        } else {
            return run_form(table, callable)
        }
    }.map_err(|err| RunErrorStack::Stream { from: Box::new(err), src: Some(callable.span.to_string()), note: Some("error while evaluating callable part of current call".into()) })?;

    match callable_value.as_ref() {
        Value::Function(func) => {
            let args = run_array(table, arguments) // Evaluates every argument
            .map_err(|err| // Throws any error
                RunErrorStack::Stream { 
                    from: Box::new(err), 
                    src: None, 
                    note: Some("error while evaluating arguments of current call".into()) 
                })?
            .into_iter().collect(); // Converts to a LinkedList<ValueRef>

            execute_function(table, func, args)
        }, 

        Value::Macro(mac) => execute_macro(table, mac, arguments),
        _ => if argc > 0 {
            Err(RunErrorStack::Top { src: Some(callable.span.to_string()), msg: format!("Passed arguments to non-functional form") })
        } else {
            Ok(ValueRef::clone(&callable_value))
        }
    }
}


#[cfg(test)]
mod tests {

}