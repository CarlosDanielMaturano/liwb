use crate::functions::*;
use crate::literals::*;
use crate::math_functions::*;
use crate::vector_manipulation::*;

use std::collections::HashMap;

pub type Variables = HashMap<String, Literal>;

const SINGLE_ARG_MATH_OPERATORS: [&'static str; 9] = [
    "sqrt", "sin", "cos", "tan", "abs", "log10", "floor", "ceil", "round",
];

const DOUBLE_ARG_MATH_OPERATORS: [&'static str; 1] = ["mod"];

const VECTOR_OPERATORS: [&'static str; 5] = ["nth", "join", "range", "map", "filter"];

pub fn eval_from_literals(literals: Vec<Literal>) -> Result<Vec<Literal>, String> {
    let mut variables: Variables = HashMap::new();
    let mut deleted_literals:  Vec<Literal> = Vec::new();
    let mut results: Vec<Literal> = Vec::new();
    for literal in literals.into_iter() {
        results.push(eval_literal(literal, &mut variables, &mut deleted_literals)?);
    }
    Ok(results)
}

pub fn eval_literal(literal: Literal, variables: &mut Variables, deleted: &mut Vec<Literal>) -> Result<Literal, String> {
    if deleted.contains(&literal) {
        return Err(format!("Trying to evaluate a deleted literal: {:?}", literal));
    }
    match literal {
        Literal::Void
        | Literal::Number(_)
        | Literal::String(_)
        | Literal::Vector(_)
        | Literal::Boolean(_) => Ok(literal),
        Literal::List(list) => eval_list(list, variables, deleted),
        Literal::Symbol(s) => {
            let Some(literal) = variables.get(&s) else {
                return Err(format!("Unknow symbol {s}"));
            };
            Ok(literal.clone())
        }
        _ => todo!("Missing literal implementation for {:?}", literal),
    }
}

fn eval_list(list: Vec<Literal>, variables: &mut Variables, deleted: &mut Vec<Literal>) -> Result<Literal, String> {
    let head = list[0].clone();
    match head {
        Literal::List(list) => eval_list(list.to_vec(), variables, deleted),
        Literal::Void => Ok(Literal::Void),
        Literal::MathOperator(_) => eval_math_operator(list, variables, deleted),
        Literal::BinaryOperator(_) => eval_binary_operator(list, variables, deleted),
        Literal::If => eval_if(list, variables, deleted),
        Literal::Vector(_) => eval_literal(head, variables, deleted),
        Literal::Symbol(s) => match s.as_str() {
            "fn" => define_function(list, variables),
            "define" => define_variable(list, variables, deleted),
            "print" => eval_print(list, variables, deleted),
            "do" => eval_do(list, variables, deleted),
            "str" => eval_str(list, variables, deleted),
            "delete" => eval_delete(list, deleted),
            s if SINGLE_ARG_MATH_OPERATORS.contains(&s) => {
                eval_operator_with_single_arg(list, variables, deleted)
            }
            s if DOUBLE_ARG_MATH_OPERATORS.contains(&s) => {
                eval_operator_with_double_argument(list, variables, deleted)
            }
            s if VECTOR_OPERATORS.contains(&s) => eval_vector_operation(list, variables, deleted),
            _ => {
                if let Some(literal) = variables.get(&s) {
                    if let Literal::Function { .. } = literal {
                        return eval_function(literal.clone(), list, variables, deleted);
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

fn eval_math_operator(list: Vec<Literal>, variables: &mut Variables, deleted: &mut Vec<Literal>) -> Result<Literal, String> {
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

    let Literal::Number(head) = eval_literal(head.clone(), variables, deleted)? else {
        return Err(format!(
            "Error. Expected head to be Literal::Number, found: {:?}",
            head
        ));
    };

    list.try_fold(Literal::Number(head), |acc, literal| {
        let literal = eval_literal(literal, variables, deleted)?;
        let Literal::Number(n) = literal else {
            return Err(format!(
                "Error. Expected Literal::Number for the literal, found {:?}",
                literal
            ));
        };
        let acc = eval_literal(acc, variables, deleted)?;
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

fn eval_binary_operator(list: Vec<Literal>, variables: &mut Variables, deleted: &mut Vec<Literal>) -> Result<Literal, String> {
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
        | Operator::BiggerOrEqualThan => eval_relation_operator(list, variables, deleted),
    }
}

fn eval_if(list: Vec<Literal>, variables: &mut Variables, deleted: &mut Vec<Literal>) -> Result<Literal, String> {
    let mut list = list.into_iter().skip(1);

    let statement = list
        .next()
        .ok_or(format!("Missing boolean statement for IF"))?;

    let left = list.next().ok_or(format!("Missing left side for IF"))?;

    let right = list.next().ok_or(format!("Missing  right side of IF"))?;

    let Literal::Boolean(statement) = eval_literal(statement.clone(), variables, deleted)? else {
        return Err(format!(
            "Error: expected Literal::Boolean. Found {:?}",
            statement
        ));
    };

    if statement {
        return eval_literal(left.clone(), variables, deleted);
    }
    return eval_literal(right.clone(), variables, deleted);
}

fn define_variable(list: Vec<Literal>, variables: &mut Variables, deleted: &mut Vec<Literal>) -> Result<Literal, String> {
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

    let literal = eval_literal(literal.clone(), variables, deleted)?;

    variables.insert(name.to_string(), literal);

    Ok(Literal::Void)
}

fn eval_relation_operator(
    list: Vec<Literal>,
    variables: &mut Variables,
    deleted: &mut Vec<Literal>
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
    let left = eval_literal(left, variables, deleted)?;

    let right = list
        .next()
        .ok_or(format!("Error. Missing left value for operation"))?;
    let right = eval_literal(right, variables, deleted)?;

    Ok(Literal::Boolean(match operator {
        Operator::Equal => left == right,
        Operator::LessThan => left < right,
        Operator::BiggerThan => left > right,
        Operator::LessOrEqualThan => left <= right,
        Operator::BiggerOrEqualThan => left >= right,
    }))
}

fn eval_print(list: Vec<Literal>, variables: &mut Variables, deleted: &mut Vec<Literal>) -> Result<Literal, String> {
    list.into_iter()
        .skip(1)
        .map(|literal| eval_literal(literal, variables, deleted))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .for_each(|literal| {
            let mut literal = literal;
            if let Literal::String(ref s) = literal {
                let regex = regex::Regex::new(r"\\(.)").unwrap();
                let string = regex
                    .replace_all(s, |capture: &regex::Captures| {
                        match &capture[1] {
                            "n" => "\n",
                            "t" => "\t",
                            "r" => "\r",
                            _ => &capture[1],
                        }
                        .to_string()
                    })
                    .to_string();

                literal = Literal::String(string);
            }
            print!("{literal} ")
        });
    Ok(Literal::Void)
}

fn eval_do(list: Vec<Literal>, variables: &mut Variables, deleted: &mut Vec<Literal>) -> Result<Literal, String> {
    list.into_iter()
        .skip(1)
        .map(|literal| eval_literal(literal, variables, deleted))
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Literal::Void)
}

fn eval_str(list: Vec<Literal>, variables: &mut Variables, deleted: &mut Vec<Literal>) -> Result<Literal, String> {
    Ok(list
        .into_iter()
        .skip(1)
        .map(|literal| eval_literal(literal, variables, deleted))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .reduce(|acc, literal| {
            let mut literal = literal;
            if let Literal::String(ref s) = literal {
                let regex = regex::Regex::new(r"\\(.)").unwrap();
                let string = regex
                    .replace_all(s, |capture: &regex::Captures| {
                        match &capture[1] {
                            "n" => "\n",
                            "t" => "\t",
                            "r" => "\r",
                            _ => &capture[1],
                        }
                        .to_string()
                    })
                    .to_string();
                literal = Literal::String(string);
            }
            let acc = acc.to_string();
            let literal = literal.to_string();
            Literal::String(format!("{acc}{literal}"))
        })
        .unwrap_or(Literal::Void))
}

fn eval_delete(list: Vec<Literal>, deleted: &mut Vec<Literal>) -> Result<Literal, String> {
    list.into_iter().for_each(|literal| {
        deleted.push(literal)
    });
    Ok(Literal::Void)
}   
