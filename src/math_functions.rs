use crate::evaluator::*;
use crate::literals::*;

pub fn eval_operator_with_single_arg(
    list: Vec<Literal>,
    variables: &mut Variables,
    deleted: &mut Vec<Literal>
) -> Result<Literal, String> {
    let mut list = list.into_iter();
    let operator = list.next().ok_or(format!("Error. Missing operator"))?;

    let Literal::Symbol(operator) = operator else {
        return Err(format!(
            "Error. Expected Literal::Symbol for operator, found: {:?}",
            operator
        ));
    };

    let left = list
        .next()
        .ok_or(format!("Error. Could not get left side for opertion."))?;

    let Literal::Number(n) = eval_literal(left.clone(), variables, deleted)? else {
        return Err(format!(
            "Error: Could not complete sqrt evaluation, expected Literal::Number, found {:?}",
            left
        ));
    };

    Ok(Literal::Number(match operator.as_str() {
        "sqrt" => n.sqrt(),
        "sin" => n.sin(),
        "cos" => n.cos(),
        "tan" => n.tan(),
        "abs" => n.abs(),
        "log10" => n.log10(),
        "floor" => n.floor(),
        "ceil" => n.ceil(),
        "round" => n.round(),
        _ => n,
    }))
}

pub fn eval_operator_with_double_argument(
    list: Vec<Literal>,
    variables: &mut Variables,
    deleted: &mut Vec<Literal>
) -> Result<Literal, String> {
    let mut list = list.into_iter();
    let operator = list.next().ok_or(format!("Error. Missing operator"))?;

    let Literal::Symbol(operator) = operator else {
        return Err(format!(
            "Error. Expected Literal::Symbol for operator, found: {:?}",
            operator
        ));
    };

    let left = list
        .next()
        .ok_or(format!("Error. Could not get left side for opertion."))?;

    let Literal::Number(left) = eval_literal(left.clone(), variables, deleted)? else {
        return Err(format!(
            "Error: Could not complete sqrt evaluation, expected Literal::Number, found {:?}",
            left
        ));
    };

    let right = list
        .next()
        .ok_or(format!("Error. Could not get left side for opertion."))?;

    let Literal::Number(right) = eval_literal(right.clone(), variables, deleted)? else {
        return Err(format!(
            "Error: Could not complete sqrt evaluation, expected Literal::Number, found {:?}",
            right
        ));
    };

    match operator.as_str() {
        "mod" => Ok(Literal::Number(left % right)),
        _ => Err(format!("Error. Unknow type of operator: {operator}")),
    }
}
