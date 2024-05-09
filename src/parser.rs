use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Void,
    List(Vec<Literal>),
    Number(f64),
    Symbol(String),
    BinaryOperator(Operator),
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Literal>, String> {
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
    let Some(Token::Lparen) = head else {
        return Err(format!("Error: Expected Lparen, found: {:?}", head));
    };

    let mut literals: Vec<Literal> = Vec::new();
    while let Some(token) = tokens.peek() {
        match token {
            Token::Symbol(s) => match s.as_str() {
                "+" => literals.push(Literal::BinaryOperator(Operator::Add)),
                "-" => literals.push(Literal::BinaryOperator(Operator::Subtract)),
                "*" => literals.push(Literal::BinaryOperator(Operator::Multiply)),
                "/" => literals.push(Literal::BinaryOperator(Operator::Divide)),
                _ => literals.push(Literal::Symbol(s.to_string())),
            },
            Token::Number(n) => literals.push(Literal::Number(*n)),
            Token::Lparen => {
                literals.push(parse_tokens(tokens)?);
            }
            Token::Rparen => break,
        }
        tokens.next();
    }
    let last = tokens.peek();
    match last {
        Some(Token::Rparen) => {
            if literals.is_empty() {
                return Ok(Literal::Void);
            }
            return Ok(Literal::List(literals));
        }
        _ => Err(format!(
            "Unclosed parenthesis somewhere. Good luck trying to find it."
        )),
    }
}
