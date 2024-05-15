#[derive(Debug, PartialEq)]
pub enum Token {
    Symbol(String),
    Number(f64),
    Lparen,
    Rparen,
    LBracket,
    RBracket,
}

pub fn lexer<'a>(source: &'a str) -> Vec<Token> {
    regex::Regex::new(r#"[^\s\[\]\(\)"']+|(\[|\]|\(|\)|"[^"]*")"#)
        .unwrap()
        .find_iter(source)
        .map(|lexeme| match lexeme.as_str() {
            "(" => Token::Lparen,
            ")" => Token::Rparen,
            "[" => Token::LBracket,
            "]" => Token::RBracket,
            lexeme => {
                if let Ok(number) = lexeme.parse::<f64>() {
                    return Token::Number(number);
                }
                Token::Symbol(lexeme.to_string())
            }
        })
        .collect::<Vec<_>>()
}
