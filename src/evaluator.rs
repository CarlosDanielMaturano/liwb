use crate::parser::{Literal, Operator};
use std::collections::HashMap;

type Variables = HashMap<String, Literal>;

pub fn eval_from_literals(literals: Vec<Literal>) -> Result<Vec<Literal>, String> {
    let mut variables: Variables = HashMap::new();
    let mut results: Vec<Literal> = Vec::new();
    for literal in literals.into_iter() {
        results.push(eval_literal(literal, &mut variables)?);
    }
    Ok(results)
}

fn eval_literal(literal: Literal, variables: &mut Variables) -> Result<Literal, String> {
    match literal {
        Literal::Void | Literal::Number(_) => Ok(literal),
        Literal::List(list) => eval_list(list, variables),
        Literal::Symbol(s) => {
            let Some(literal) = variables.get(&s) else {
                return Err(format!("Unknow symbol {s}"));
            };
            Ok(literal.clone())
        }
        _ => todo!("Missing literal implementation"),
    }
}

fn eval_list(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let head = &list[0];
    match head {
        Literal::List(list) => eval_list(list.to_vec(), variables),
        Literal::Void => Ok(Literal::Void),
        Literal::BinaryOperator(_) => eval_binary(list, variables),
        Literal::Symbol(s) => match s.as_str() {
            "define" => define_variable(list, variables),
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn eval_binary(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let mut list = list.into_iter();
    let operator = list.next();
    let Some(Literal::BinaryOperator(operator)) = operator else {
        panic!("{:?}", operator)
    };
    list.reduce(|acc, literal| {
        let literal = eval_literal(literal, variables);
        let Ok(Literal::Number(number)) = literal else {
            panic!(
                "Error. Expected Literal::Number for the literal, found {:?}",
                literal
            )
        };
        let acc = eval_literal(acc, variables);
        let Ok(Literal::Number(acc)) = acc else {
            panic!(
                "Error. Expected Literal::Number for the acumulator, found {:?}",
                acc
            )
        };
        return Literal::Number(match operator {
            Operator::Add => acc + number,
            Operator::Subtract => acc - number,
            Operator::Multiply => acc * number,
            Operator::Divide => acc / number,
        });
    })
    .ok_or_else(|| format!("Error: Could not complete the operation!"))
}

fn define_variable(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let [Literal::Symbol(name), literal] = &list[1..=2] else {
        return Err(format!("Missing arguments for variable definition"));
    };
    let literal = eval_literal(literal.clone(), variables)?;
    variables.insert(name.to_string(), literal);
    Ok(Literal::Void)
}
