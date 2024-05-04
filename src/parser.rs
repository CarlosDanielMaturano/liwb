use crate::lexer::Token;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub enum Object {
    Void,
    List(Vec<Object>),
    Number(f64),
    BinaryOperator(Operator),
}

pub fn parse(tokens: Vec<Token>) -> Result<Object, String> {
    if tokens.is_empty() {
        return Ok(Object::Void);
    }
    parse_tokens(&mut tokens.into_iter().peekable())
}

type PeekableTokens = Peekable<std::vec::IntoIter<Token>>;
fn parse_tokens(tokens: &mut PeekableTokens) -> Result<Object, String> {
    let head = tokens.next();

    let Some(Token::Lparen) = head else {
        return Err(format!("Error: Expected Lparen, found: {:?}", head));
    };

    let mut objects: Vec<Object> = Vec::new();

    while let Some(token) = tokens.peek() {
        match token {
            Token::Symbol(s) => match s.as_str() {
                "+" => objects.push(Object::BinaryOperator(Operator::Add)),
                "-" => objects.push(Object::BinaryOperator(Operator::Subtract)),
                "*" => objects.push(Object::BinaryOperator(Operator::Multiply)),
                "/" => objects.push(Object::BinaryOperator(Operator::Divide)),
                _ => todo!("Missing symbols"),
            },
            Token::Number(n) => objects.push(Object::Number(*n)),
            Token::Lparen => objects.push(parse_tokens(tokens)?),
            Token::Rparen => break,
        }
        tokens.next();
    }
    if objects.is_empty() {
        return Ok(Object::Void);
    }
    return Ok(Object::List(objects));
}
