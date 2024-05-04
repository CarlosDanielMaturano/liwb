#[derive(Debug, PartialEq)]
pub enum Token {
    Symbol(String),
    Number(f64),
    Lparen,
    Rparen,
}

pub fn lexer<'a>(source: &'a str) -> Vec<Token> {
    source
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|lexeme| match lexeme {
            "(" => Token::Lparen,
            ")" => Token::Rparen,
            _ => {
                if let Ok(number) = lexeme.parse::<f64>() {
                    return Token::Number(number);
                }
                Token::Symbol(lexeme.to_string())
            }
        })
        .collect::<Vec<_>>()
}
