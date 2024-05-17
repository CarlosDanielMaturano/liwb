use crate::evaluator::*;
use crate::literals::*;

pub fn single_operator(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let mut list = list.into_iter();
    let operator = list.next().ok_or(format!("Error. Missing opeator"))?;

    let Literal::Symbol(operator) = operator else {
        return Err(format!(
            "Error. Expected Literal::Symbol for operator, found: {:?}",
            operator
        ));
    };

    let left = list
        .next()
        .ok_or(format!("Error. Could not get left side for opertion."))?;

    let Literal::Number(n) = eval_literal(left.clone(), variables)? else {
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
