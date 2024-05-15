#[derive(Debug, PartialEq)]
pub enum Token {
    Symbol(String),
    Number(f64),
    Lparen,
    Rparen,
}

pub fn lexer<'a>(source: &'a str) -> Vec<Token> {
    regex::Regex::new(r#""[^"]*"|\S+"#)
        .unwrap()
        .find_iter(source.replace("(", " ( ").replace(")", " ) ").as_str())
        .map(|lexeme| match lexeme.as_str() {
            "(" => Token::Lparen,
            ")" => Token::Rparen,
            lexeme => {
                if let Ok(number) = lexeme.parse::<f64>() {
                    return Token::Number(number);
                }
                Token::Symbol(lexeme.to_string())
            }
        })
        .collect::<Vec<_>>()
}
