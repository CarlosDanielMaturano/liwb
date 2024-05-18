use crate::functions::*;
use crate::literals::*;
use crate::math_functions::*;
use crate::vector_manipulation::*;

use std::collections::HashMap;

pub type Variables = HashMap<String, Literal>;

const SINGLE_MATH_OPERATORS: [&'static str; 9] = [
    "sqrt", "sin", "cos", "tan", "abs", "log10", "floor", "ceil", "round",
];

const VECTOR_OPERATORS: [&'static str; 4] = ["nth", "join", "range", "map"];

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
        Literal::Void
        | Literal::Number(_)
        | Literal::String(_)
        | Literal::Vector(_)
        | Literal::Boolean(_) => Ok(literal),
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
    let head = list[0].clone();
    match head {
        Literal::List(list) => eval_list(list.to_vec(), variables),
        Literal::Void => Ok(Literal::Void),
        Literal::MathOperator(_) => eval_math_operator(list, variables),
        Literal::BinaryOperator(_) => eval_binary_operator(list, variables),
        Literal::If => eval_if(list, variables),
        Literal::Vector(_) => eval_literal(head, variables),
        Literal::Symbol(s) => match s.as_str() {
            "fn" => define_function(list, variables),
            "define" => define_variable(list, variables),
            s if SINGLE_MATH_OPERATORS.contains(&s) => single_operator(list, variables),
            s if VECTOR_OPERATORS.contains(&s) => eval_operation(list, variables),
            _ => {
                if let Some(literal) = variables.get(&s) {
                    if let Literal::Function { .. } = literal {
                        return eval_function(literal.clone(), list, variables);
                    }
                    Ok(literal.clone())
                } else {
                    return Err(format!("Unknow symbol: {s}"));
                }
            }
        },
        Literal::String(_) | Literal::Boolean(_) | Literal::Number(_) => Ok(head),
        Literal::Function { .. } => Ok(head),
    }
}

fn eval_math_operator(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let mut list = list.into_iter();
    let operator = list.next();

    let Some(Literal::MathOperator(operator)) = operator else {
        return Err(format!(
            "Error. Expected Literal::MathOperator, found: {:?}",
            operator
        ));
    };

    let head = list
        .next()
        .ok_or(format!("Error. Could not get the head of the operation."))?;

    let Literal::Number(head) = eval_literal(head.clone(), variables)? else {
        return Err(format!(
            "Error. Expected head to be Literal::Number, found: {:?}",
            head
        ));
    };

    list.try_fold(Literal::Number(head), |acc, literal| {
        let literal = eval_literal(literal, variables)?;
        let Literal::Number(n) = literal else {
            return Err(format!(
                "Error. Expected Literal::Number for the literal, found {:?}",
                literal
            ));
        };
        let acc = eval_literal(acc, variables)?;
        let Literal::Number(acc) = acc else {
            return Err(format!(
                "Error. Expected Literal::Number for the acumulator, found {:?}",
                acc
            ));
        };
        return Ok(Literal::Number(match operator {
            MathOperators::Add => acc + n,
            MathOperators::Subtract => acc - n,
            MathOperators::Multiply => acc * n,
            MathOperators::Divide => acc / n,
        }));
    })
}

fn eval_binary_operator(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let mut list = list.into_iter();
    let operator = list.next();
    let Some(Literal::BinaryOperator(operator)) = operator else {
        return Err(format!(
            "Error. Expected Literal::BinaryOperator, found: {:?}",
            operator
        ));
    };
    let list = vec![Literal::BinaryOperator(operator.clone())]
        .into_iter()
        .chain(list)
        .collect();

    match operator {
        Operator::Equal
        | Operator::LessThan
        | Operator::BiggerThan
        | Operator::LessOrEqualThan
        | Operator::BiggerOrEqualThan => eval_relation_operator(list, variables),
    }
}

fn eval_if(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let mut list = list.into_iter().skip(1);

    let statement = list
        .next()
        .ok_or(format!("Missing boolean statement for IF"))?;

    let left = list.next().ok_or(format!("Missing left side for IF"))?;

    let right = list.next().ok_or(format!("Missing  right side of IF"))?;

    let Literal::Boolean(statement) = eval_literal(statement.clone(), variables)? else {
        return Err(format!(
            "Error: expected Literal::Boolean. Found {:?}",
            statement
        ));
    };

    if statement {
        return eval_literal(left.clone(), variables);
    }
    return eval_literal(right.clone(), variables);
}

fn define_variable(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let mut list = list.into_iter().skip(1);

    let name = list
        .next()
        .ok_or(format!("Error. Missing variable name for definition"))?;

    let Literal::Symbol(name) = name else {
        return Err(format!(
            "Error. Expected Literal::Symbol for variable name, found: {:?}",
            name
        ));
    };

    let literal = list.next().ok_or(format!(
        "Error. Missing literal value for variable definition."
    ))?;

    let literal = eval_literal(literal.clone(), variables)?;

    variables.insert(name.to_string(), literal);

    Ok(Literal::Void)
}

fn eval_relation_operator(
    list: Vec<Literal>,
    variables: &mut Variables,
) -> Result<Literal, String> {
    let list_size = list.len();
    if list_size != 3 {
        return Err(format!(
            "Error. Wrong number of arguments for relational operation. Expected 3, found: {list_size}"
        ));
    }

    let mut list = list.into_iter();

    let operator = list.next().ok_or(format!(
        "Error. Missing operator for the relational operation"
    ))?;

    let Literal::BinaryOperator(operator) = operator else {
        return Err(format!(
            "Error. Expected Literal::BinaryOperator for the operator, found: {:?} ",
            list
        ));
    };

    let left = list.next().ok_or(format!(
        "Error. Missing left value for the relational operation"
    ))?;
    let left = eval_literal(left, variables)?;

    let right = list
        .next()
        .ok_or(format!("Error. Missing left value for operation"))?;
    let right = eval_literal(right, variables)?;

    Ok(Literal::Boolean(match operator {
        Operator::Equal => left == right,
        Operator::LessThan => left < right,
        Operator::BiggerThan => left > right,
        Operator::LessOrEqualThan => left <= right,
        Operator::BiggerOrEqualThan => left >= right,
    }))
}
