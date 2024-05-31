use crate::lexer::Token;
use crate::literals::*;

pub fn parser(tokens: Vec<Token>) -> Result<Vec<Literal>, String> {
    if tokens.is_empty() {
        return Ok(vec![Literal::Void]);
    }
    let mut tokens = tokens.into_iter().peekable();
    let mut literals: Vec<Literal> = Vec::new();
    while let Some(_) = tokens.peek() {
        literals.push(parse_tokens(&mut tokens)?);
        tokens.next();
    }
    return Ok(literals);
}

type PeekableTokens = std::iter::Peekable<std::vec::IntoIter<Token>>;
fn parse_tokens(tokens: &mut PeekableTokens) -> Result<Literal, String> {
    let head = tokens.next();
    let Some(Token::Lparen | Token::LBracket) = head else {
        return Err(format!("Error: Expected Lparen, found: {:?}", head));
    };

    let mut literals: Vec<Literal> = Vec::new();
    while let Some(token) = tokens.peek() {
        match token {
            Token::Symbol(s) => match s.as_str() {
                "+" => literals.push(Literal::MathOperator(MathOperators::Add)),
                "-" => literals.push(Literal::MathOperator(MathOperators::Subtract)),
                "*" => literals.push(Literal::MathOperator(MathOperators::Multiply)),
                "/" => literals.push(Literal::MathOperator(MathOperators::Divide)),
                "=" => literals.push(Literal::BinaryOperator(Operator::Equal)),
                "<" => literals.push(Literal::BinaryOperator(Operator::LessThan)),
                ">" => literals.push(Literal::BinaryOperator(Operator::BiggerThan)),
                "<=" => literals.push(Literal::BinaryOperator(Operator::LessOrEqualThan)),
                ">=" => literals.push(Literal::BinaryOperator(Operator::BiggerOrEqualThan)),
                "!=" => literals.push(Literal::BinaryOperator(Operator::NotEqual)),
                "true" => literals.push(Literal::Boolean(true)),
                "false" => literals.push(Literal::Boolean(false)),
                "if" => literals.push(Literal::If),
                _ => {
                    if s.contains("\"") {
                        if s.chars().into_iter().filter(|c| *c == '"').count() != 2 {
                            return Err(String::from(
                                "You have a invalid string literal somewhere, good luck trying to find it!",
                            ));
                        }
                        literals.push(Literal::String(s.replace("\"", "").to_string()));
                    } else {
                        literals.push(Literal::Symbol(s.to_string()));
                    }
                }
            },
            Token::LBracket => {
                let result = match parse_tokens(tokens)? {
                    Literal::List(list) => list,
                    Literal::Void => vec![],
                    result => {
                        return Err(format!(
                            "Expected LIteral::List or Literal::Void for Vector. found: {:?}",
                            result
                        ))
                    }
                };
                literals.push(Literal::Vector(result));
            }
            Token::Number(n) => literals.push(Literal::Number(*n)),
            Token::Lparen => {
                literals.push(parse_tokens(tokens)?);
            }
            Token::Rparen | Token::RBracket => break,
        }
        tokens.next();
    }
    let last = tokens.peek();
    match last {
        Some(Token::Rparen | Token::RBracket) => {
            if literals.is_empty() {
                dbg!(literals);
                return Ok(Literal::Void);
            }
            return Ok(Literal::List(literals));
        }
        _ => Err(format!(
            "Unclosed parenthesis somewhere. Good luck trying to find it."
        )),
    }
}
