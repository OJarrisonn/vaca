use std::error::Error;

use vaca_core::build::{error::BuildErrorStack, form::{function::Function, Expr, Form}, symbol::Symbol};

use crate::BuildResult;

use super::table::TrackTable;

/// Takes a form and does Symbol validation, it means, check if all the symbols being used are defined
pub fn validate_form(track: &mut TrackTable, form: &Form) -> BuildResult<()> {
    match form.expr() {
        Expr::Symbol(s) => track.validate(s),
        Expr::AssignmentList(list) => validate_assingment_list(track, form, list),
        Expr::Scope(forms) => validate_forms(track, form, forms),
        Expr::Function(function) => validate_function(track, form, function),
        Expr::Macro(_) => todo!(),
        Expr::Call(_) => todo!(),
        Expr::Array(_) => todo!(),
        _ => Ok(())
    }
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

/// Takes a function and validates its captures and its body
fn validate_function(track: &mut TrackTable, form: &Form, function: &Function) -> BuildResult<()> {
    let captures = function.captures();
    let parameters = function.parameters();
    let body= function.body();

    let cres = if let Some(captures) = captures {
        let mut errs = captures.iter()
            .map(|symbol| track.validate(symbol))
            .filter(Result::is_err)
            .collect::<Vec<_>>();

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
    } else {
        Ok(())
    };

    // Fake evaluate the body
    track.create_scope();
    parameters.iter().for_each(|p| { let _ = track.assign(p); });

    let bres = validate_form(track, body);

    track.drop_scope();

    match (cres, bres) {
        (Ok(_), Ok(_)) => Ok(()),
        (Ok(_), Err(berr)) => Err(berr),
        (Err(cerr), Ok(_)) => Err(cerr),
        (Err(cerr), Err(berr)) => Err(BuildErrorStack::MultiStream { from: vec![Box::new(cerr), Box::new(berr)], src: form.span().to_string(), note: None })
    }
}




