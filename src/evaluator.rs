use crate::{
    math_functions::*,
    parser::{Literal, MathOperators, Operator},
    vector_manipulation::*,
};
use std::collections::HashMap;

pub type Variables = HashMap<String, Literal>;

const SINGLE_MATH_OPERATORS: [&'static str; 9] = [
    "sqrt", "sin", "cos", "tan", "abs", "log10", "floor", "ceil", "round",
];

const VECTOR_OPERATORS: [&'static str; 2] = ["nth", "push"];

pub fn eval_from_literals(literals: Vec<Literal>) -> Result<Vec<Literal>, String> {
    let mut variables: Variables = HashMap::new();
    let mut results: Vec<Literal> = Vec::new();
    for literal in literals.into_iter() {
        results.push(eval_literal(literal, &mut variables)?);
    }
    Ok(results)
}

pub fn eval_literal(literal: Literal, variables: &mut Variables) -> Result<Literal, String> {
    match literal {
        Literal::Void | Literal::Number(_) | Literal::String(_) | Literal::Vector(_) => Ok(literal),
        Literal::List(list) => eval_list(list, variables),
        Literal::Symbol(s) => {
            let Some(literal) = variables.get(&s) else {
                return Err(format!("Unknow symbol {s}"));
            };
            Ok(literal.clone())
        }
        _ => todo!("Missing literal implementation for {:?}", literal),
    }
}

fn eval_list(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let head = &list[0];
    match head {
        Literal::List(list) => eval_list(list.to_vec(), variables),
        Literal::Void => Ok(Literal::Void),
        Literal::MathOperator(_) => eval_math_operator(list, variables),
        Literal::BinaryOperator(_) => eval_binary_operator(list, variables),
        Literal::If => eval_if(list, variables),
        Literal::Symbol(s) => match s.as_str() {
            "define" => define_variable(list, variables),
            s if SINGLE_MATH_OPERATORS.contains(&s) => single_operator(list, variables),
            s if VECTOR_OPERATORS.contains(&s) => eval_operation(list, variables),
            _ => {
                if let Some(literal) = variables.get(s) {
                    Ok(literal.clone())
                } else {
                    return Err(format!("Unknow symbol: {s}"));
                }
            }
        },
        _ => todo!(),
    }
}

fn eval_math_operator(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let mut list = list.into_iter();
    let operator = list.next();
    let Some(Literal::MathOperator(operator)) = operator else {
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
            MathOperators::Add => acc + number,
            MathOperators::Subtract => acc - number,
            MathOperators::Multiply => acc * number,
            MathOperators::Divide => acc / number,
        });
    })
    .ok_or_else(|| format!("Error: Could not complete the operation!"))
}

fn eval_binary_operator(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let mut list = list.into_iter();
    let operator = list.next();
    let Some(Literal::BinaryOperator(operator)) = operator else {
        panic!("{:?}", operator)
    };
    match operator {
        Operator::Equal => eval_equal(list.collect(), variables),
    }
}

fn eval_equal(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let [left, right] = &list[..2] else { todo!() };
    let left = eval_literal(left.clone(), variables);
    let right = eval_literal(right.clone(), variables);
    Ok(Literal::Boolean(left == right))
}

fn eval_if(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let [_, statement, left, right] = &list[..4] else {
        todo!()
    };
    let Ok(Literal::Boolean(statement)) = eval_literal(statement.clone(), variables) else {
        panic!("Error: expected Literal::Boolean. Found {:?}", statement)
    };
    let left = eval_literal(left.clone(), variables)?;
    let right = eval_literal(right.clone(), variables)?;
    if statement {
        Ok(left)
    } else {
        Ok(right)
    }
}

fn define_variable(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let [Literal::Symbol(name), literal] = &list[1..=2] else {
        return Err(format!("Missing arguments for variable definition"));
    };
    let literal = eval_literal(literal.clone(), variables)?;
    variables.insert(name.to_string(), literal);
    Ok(Literal::Void)
}
