use std::env::vars_os;

use crate::evaluator::*;
use crate::parser::Literal;

pub fn eval_operation(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let Literal::Symbol(operator) = &list[0] else {
        todo!();
    };
    match operator.as_str() {
        "nth" => eval_nth(list, variables),
        _ => todo!(),
    }
}

pub fn eval_nth(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    if list.len() != 3 {
        return Err(format!("Missing arguments for nth"))
    }
    let [_, vector_name, Literal::Number(index)] = &list[..3] else {
        panic!("{:?}", list);
    };
    let Literal::Vector(v) = eval_literal(vector_name.clone(), variables)? else {
        todo!()
    };
    if *index < 0.0 {
        todo!()
    }
    let index: usize = index.round() as usize;
    Ok(v.into_iter()
        .nth(index)
        .unwrap_or(Literal::Void))
}
