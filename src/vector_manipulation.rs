use crate::evaluator::*;
use crate::literals::*;

pub fn eval_operation(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let operator = &list[0];
    let Literal::Symbol(operator) = operator else {
        return Err(format!(
            "Error. Expected a Literal::Symbol for the vector operation, found: {:?}",
            operator
        ));
    };
    match operator.as_str() {
        "nth" => eval_nth(list, variables),
        "join" => eval_join(list, variables),
        "range" => eval_range(list, variables),
        _ => todo!(),
    }
}

pub fn eval_nth(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    if list.len() != 3 {
        return Err(format!("Missing arguments for nth"));
    }
    let mut list = list.into_iter().skip(1);
    let vector_name = list
        .next()
        .ok_or(format!("Error. Could not get the name of the vector!"))?;

    let index = list
        .next()
        .ok_or(format!("Error. Could not get the index!"))?;

    let Ok(Literal::Number(index)) = eval_literal(index.clone(), variables) else {
        return Err(format!(
            "Error. Expected Literal::Number for index, found: {:?}",
            index
        ));
    };

    let v = eval_literal(vector_name.clone(), variables)?;
    let Literal::Vector(v) = v else {
        return Err(format!("Error. Expected Literal::Vector, found: {:?}", v));
    };
    let index: usize = index.round() as usize;
    Ok(v.into_iter().nth(index).unwrap_or(Literal::Void))
}

pub fn eval_join(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let mut list = list.into_iter().skip(1);
    let vector_name = list
        .next()
        .ok_or(format!("Error. Could not get the name of the vector!"))?;
    let vector = eval_literal(vector_name, variables)?;
    let Literal::Vector(vector) = vector else {
        return Err(format!(
            "Error. Expected join fist argument to bet Literal::Vector, found: {:?}",
            vector
        ));
    };

    let list = list
        .map(|literal| eval_literal(literal, variables))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter();

    Ok(Literal::Vector(
        vector.into_iter().chain(list).collect::<Vec<_>>(),
    ))
}

pub fn eval_range(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let mut list = list.into_iter().skip(1);
    let start = eval_literal(
        list.next()
            .ok_or(format!("Error. Missing start agurment for range function."))?,
        variables,
    )?;
    let Literal::Number(start) = start else {
        return Err(format!(
            "Error. Expected start to be Literal::Number, found: {:?}", 
            start
        ))
    };

    let end = eval_literal(
        list.next()
            .ok_or(format!("Error. Missing end agurment for range function."))?,
        variables,
    )?;
    let Literal::Number(end) = end else {
        return Err(format!(
            "Error. Expected end to be Literal::Number, found: {:?}", 
            start
        ))
    };
    let start = start as i64;
    let end = end as i64;
    let range = (start..=end)
        .map(|number| Literal::Number(number as f64))
        .collect::<Vec<_>>();

    Ok(Literal::Vector(range))
}
