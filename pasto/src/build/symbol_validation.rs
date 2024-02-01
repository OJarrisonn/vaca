use std::error::Error;

use vaca_core::build::{error::BuildErrorStack, form::{function::Function, Expr, Form, macros::Macro, call::Call}, program::Program, symbol::Symbol};

use crate::BuildResult;

use super::{populate_track, table::TrackTable};

pub fn validate_program(program: &Program) -> BuildResult<()> {
    let forms = program.forms();
    let mut track = TrackTable::new();
    track.create_scope();
    populate_track(&mut track);

    let mut errs = forms.iter()
        .map(|form| validate_form(&mut track, form))
        .filter(Result::is_err)
        .map(Result::unwrap_err)
        .collect::<Vec<BuildErrorStack>>();

    if errs.is_empty() {
        Ok(())
    } else if errs.len() == 1 {
        Err(errs.pop().unwrap())
    } else {
        Err(BuildErrorStack::MultiStream { from: errs.into_iter().map(|err| Box::new(err) as Box<dyn Error>).collect(), src: "".into(), note: Some("The current program uses undefined symbols".into()) })
    }

}

/// Takes a form and does Symbol validation, it means, check if all the symbols being used are defined
pub fn validate_form(track: &mut TrackTable, form: &Form) -> BuildResult<()> {
    match form.expr() {
        Expr::Symbol(s) => track.validate(s),
        Expr::AssignmentList(list) => validate_assingment_list(track, form, list),
        Expr::Scope(forms) => validate_forms(track, form, forms),
        Expr::Function(function) => validate_function(track, form, function),
        Expr::Macro(macrodef) => validate_macro(track, form, macrodef),
        Expr::Call(call) => validate_call(track, form, call),
        Expr::Array(array) => validate_array(track, form, array),
        _ => Ok(())
    }
}

/// Takes a list of forms and validates all of them, by checking the symbols
fn validate_forms(track: &mut TrackTable, form: &Form, forms: &Vec<Form>) -> BuildResult<()> {
    track.create_scope();
    let mut errs = forms.iter()
        .map(|form| validate_form(track, form))
        .filter(Result::is_err)
        .collect::<Vec<_>>();

    let res = if errs.is_empty() {
        Ok(())
    } else if errs.len() == 1{
        Err(BuildErrorStack::Stream { from: Box::new(errs.pop().unwrap().unwrap_err()), src: form.span().to_string(), note: None })
    } else {
        Err(BuildErrorStack::MultiStream { 
            from: errs.into_iter()
                .map(|err| Box::new(err.unwrap_err()) as Box<dyn Error>)
                .collect(), 
            src: form.span().to_string(), 
            note: None 
        })
    };

    track.drop_scope();
    res
}

/// Takes an assignment list and validates it, it means, check if the symbols provided can be assigned and if the forms passed are also symbol valid
fn validate_assingment_list(track: &mut TrackTable, form: &Form, list: &Vec<(Symbol, Form)>) -> BuildResult<()> {
    let (_, mut errs): (Vec<_>, Vec<_>) = list.iter()
        .map(|(symbol, form)| {
            let sres = track.assign(symbol);
            let fres = validate_form(track, form);

            match (sres, fres) {
                (Ok(_), Ok(_)) => Ok(()),
                (Ok(_), Err(ferr)) => Err(ferr),
                (Err(serr), Ok(_)) => Err(serr),
                (Err(serr), Err(ferr)) => Err(BuildErrorStack::MultiStream { from: vec![Box::new(serr), Box::new(ferr)], src: form.span().to_string(), note: None })
            }
        })
        .partition(Result::is_ok);

    if errs.is_empty() {
        Ok(())
    } else if errs.len() == 1{
        Err(BuildErrorStack::Stream { from: Box::new(errs.pop().unwrap().unwrap_err()), src: form.span().to_string(), note: None })
    } else {
        Err(BuildErrorStack::MultiStream { 
            from: errs.into_iter()
                .map(|err| Box::new(err.unwrap_err()) as Box<dyn Error>)
                .collect(), 
            src: form.span().to_string(), 
            note: None 
        })
    }       
}

/// Takes a function and validates its captures and its body
fn validate_function(track: &mut TrackTable, form: &Form, function: &Function) -> BuildResult<()> {
    let parameters = function.parameters();
    let body= function.body();

    // Fake evaluate the body
    track.create_scope();
    parameters.iter().for_each(|p| { let _ = track.assign(p); });

    let bres = validate_form(track, body);

    track.drop_scope();

    bres.map_err(|err| BuildErrorStack::Stream { from: Box::new(err), src: form.span().to_string(), note: Some("Use of undefined symbols in the macro body".into()) })
}

/// Takes a macro and validates its body
fn validate_macro(track: &mut TrackTable, form: &Form, macrodef: &Macro) -> BuildResult<()> {
    let parameters = macrodef.parameters();
    let body= macrodef.body();

    // Fake evaluate the body
    track.create_scope();
    parameters.iter().for_each(|p| { let _ = track.assign(p); });

    let bres = validate_form(track, body);

    track.drop_scope();

    bres.map_err(|err| BuildErrorStack::Stream { from: Box::new(err), src: form.span().to_string(), note: Some("Use of undefined symbols in the macro body".into()) })
}

/// Takes a call and validates both the callable and the arguments
fn validate_call(track: &mut TrackTable, form: &Form, call: &Call) -> BuildResult<()> {
    let callable = call.callable();
    let arguments = call.arguments();

    let cres = validate_form(track, &callable);
    let ares = validate_forms(track, form, arguments);

    match (cres, ares) {
        (Ok(_), Ok(_)) => Ok(()),
        (Ok(_), Err(aerr)) => Err(aerr),
        (Err(cerr), Ok(_)) => Err(cerr),
        (Err(cerr), Err(aerr)) => Err(BuildErrorStack::MultiStream { from: vec![Box::new(cerr), Box::new(aerr)], src: form.span().to_string(), note: Some("Use of undefined symbols in the function call".into()) }),
    }
}

/// Takes an array and validates all the elements
fn validate_array(track: &mut TrackTable, form: &Form, array: &Vec<Form>) -> BuildResult<()> {
    validate_forms(track, form, array)
        .map_err(|err| BuildErrorStack::Stream { 
            from: Box::new(err), 
            src: form.span().to_string(), 
            note: Some("Use of undefined symbols inside an array".into()) 
        })
}

