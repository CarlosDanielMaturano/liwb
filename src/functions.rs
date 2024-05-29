use crate::evaluator::*;
use crate::literals::*;

pub fn define_function(list: Vec<Literal>, variables: &mut Variables) -> Result<Literal, String> {
    let mut list = list.into_iter().skip(1);
    let name = list
        .next()
        .ok_or(format!("Error. Could not get the function name"))?;

    let Literal::Symbol(name) = name else {
        return Err(format!(
            "Error. Expected Literal::Symbol to be function name, found: {:?}",
            name
        ));
    };

    let args = list
        .next()
        .ok_or(format!("Error. Could not get the function argument name"))?;

    let Literal::Vector(args) = args else {
        return Err(format!(
            "Error. Expected Literal::Symbol to be function argument name, found: {:?}",
            args
        ));
    };

    let args = args
        .into_iter()
        .filter(|literal| *literal != Literal::Void)
        .map(|arg| match arg {
            Literal::Symbol(s) => Ok(s),
            _ => Err(format!(
                "Error. Expected Literal::Symbol type for argument name, found: {:?}",
                arg
            )),
        })
        .collect::<Result<Vec<_>, String>>()?;

    let body = list
        .next()
        .ok_or(format!("Error. Missing body of the function."))?;

    let function = Literal::Function {
        name: name.clone(),
        args,
        body: Box::new(body),
    };

    variables.insert(name, function);
    return Ok(Literal::Void);
}

pub fn eval_function(
    function: Literal,
    list: Vec<Literal>,
    variables: &mut Variables,
    deleted: &mut Vec<Literal>
) -> Result<Literal, String> {
    let values = list
        .into_iter()
        .map(|literal| eval_literal(literal, variables, deleted))
        .skip(1)
        .collect::<Result<Vec<_>, String>>()?;

    let Literal::Function { args, body, .. } = function else {
        return Err(format!("Error. Expected function, found: {:?}", function));
    };

    if values.len() != args.len() {
        return Err(format!(
            "Error. The number of arguments that the funciton requires differ from the passed"
        ));
    }

    let mut local_variables: Variables = variables.clone();

    args.into_iter()
        .zip(values.into_iter())
        .for_each(|(key, value)| {
            local_variables.insert(key, value);
        });

    eval_literal(*body, &mut local_variables, deleted)
}
