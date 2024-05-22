use crate::evaluator::*;
use crate::literals::*;

pub fn eval_vector_operation(
    list: Vec<Literal>,
    variables: &mut Variables,
) -> Result<Literal, String> {
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
        "map" => eval_map(list, variables),
        "filter" => eval_filter(list, variables),
        operator => return Err(format!("Unknow type of vector operation {operator}")),
    }
}

fn eval_nth(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
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

fn eval_join(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
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

fn eval_range(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
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
        ));
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
        ));
    };
    let start = start as i64;
    let end = end as i64;
    let range = (start..=end)
        .map(|number| Literal::Number(number as f64))
        .collect::<Vec<_>>();

    Ok(Literal::Vector(range))
}

fn eval_map(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    if list.len() != 3 {
        return Err(format!(
            "Erro. Wrong number of arguments passed to map. Expected 3, found {}",
            list.len()
        ));
    }
    let mut list = list.into_iter().skip(1);

    let function_name = list
        .next()
        .ok_or(format!("Error. Missing function parameter for map."))?;

    let Literal::Symbol(s) = &function_name else {
        return Err(format!(
            "Error. Expected Literal::Symbol for function name in map, found: {:?}",
            function_name
        ));
    };
    let function = variables.get(s);
    let Some(Literal::Function { .. }) = function else {
        return Err(format!(
            "Error. Expected Literal::Function, found: {:?}",
            function
        ));
    };

    let vector = list
        .next()
        .ok_or(format!("Error. Missing vector parameter for map."))?;

    let Literal::Vector(vector) = eval_literal(vector.clone(), variables)? else {
        return Err(format!(
            "Error. Expected Literal::Vector, found: {:?}",
            vector
        ));
    };

    Ok(Literal::Vector(
        vector
            .into_iter()
            .map(|parameter| {
                let list = Literal::List(vec![function_name.clone(), parameter]);
                eval_literal(list, variables)
            })
            .filter(|literal| Ok(Literal::Void) != *literal)
            .collect::<Result<Vec<_>, String>>()?,
    ))
}

fn eval_filter(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    if list.len() != 3 {
        return Err(format!(
            "Erro. Wrong number of arguments passed to filter. Expected 3, found {}",
            list.len()
        ));
    }
    let mut list = list.into_iter().skip(1);

    let function_name = list
        .next()
        .ok_or(format!("Error. Missing function parameter for map."))?;

    let Literal::Symbol(s) = &function_name else {
        return Err(format!(
            "Error. Expected Literal::Symbol for function name in filter, found: {:?}",
            function_name
        ));
    };
    let function = variables.get(s);
    let Some(Literal::Function { .. }) = function else {
        return Err(format!(
            "Error. Expected Literal::Function, found: {:?}",
            function
        ));
    };

    let vector = list
        .next()
        .ok_or(format!("Error. Missing vector parameter for filter."))?;

    let Literal::Vector(vector) = eval_literal(vector.clone(), variables)? else {
        return Err(format!(
            "Error. Expected Literal::Vector, found: {:?}",
            vector
        ));
    };
    Ok(Literal::Vector(
        vector
            .into_iter()
            .filter(|parameter| {
                let list = Literal::List(vec![function_name.clone(), parameter.clone()]);
                Ok(Literal::Boolean(true)) == eval_literal(list, variables)
            })
            .collect::<Vec<_>>(),
    ))
}
