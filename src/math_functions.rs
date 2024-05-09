use crate::evaluator::*;
use crate::parser::{Literal, Operator};

pub const SINGLE_OPERATORS: [&'static str; 9] = [
    "sqrt", "sin", "cos", "tan", "abs", "log10", "floor", "ceil", "round",
];

pub fn single_operator(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let [Literal::Symbol(s), left] = &list[..=1] else {
        panic!()
    };
    let left = eval_literal(left.clone(), variables)?;
    let Literal::Number(n) = left else {
        return Err(format!(
            "Error: Could not complete sqrt evaluation, expected Literal::Number, found {:?}",
            left
        ));
    };
    if !SINGLE_OPERATORS.contains(&s.as_str()) {
        panic!("Error: Unknow operator {s}")
    }
    Ok(Literal::Number(match s.as_str() {
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
