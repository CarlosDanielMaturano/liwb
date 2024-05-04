use liwb::lexer::*;

#[test]
fn empty_source_returns_empty_vector() {
    let source = "";
    assert_eq!(lexer(source), vec![]);
}

#[test]
fn basic_arithmetic_operation() {
    let source = "(+ 1 2 (- 3 (* 4 ( / 5 6))))";
    assert_eq!(
        lexer(source),
        vec![
            Token::Lparen,
            Token::Symbol("+".to_string()),
            Token::Number(1.0),
            Token::Number(2.0),
            Token::Lparen,
            Token::Symbol("-".to_string()),
            Token::Number(3.0),
            Token::Lparen,
            Token::Symbol("*".to_string()),
            Token::Number(4.0),
            Token::Lparen,
            Token::Symbol("/".to_string()),
            Token::Number(5.0),
            Token::Number(6.0),
            Token::Rparen,
            Token::Rparen,
            Token::Rparen,
            Token::Rparen,
        ]
    );
}

#[test]
fn arithmetic_operation_with_multiple_parameters() {
    let source = "(+ 1 2 3 4 5 (+ 6 7 8 9))";
    assert_eq!(
        lexer(source),
        vec![
            Token::Lparen,
            Token::Symbol("+".to_string()),
            Token::Number(1.0),
            Token::Number(2.0),
            Token::Number(3.0),
            Token::Number(4.0),
            Token::Number(5.0),
            Token::Lparen,
            Token::Symbol("+".to_string()),
            Token::Number(6.0),
            Token::Number(7.0),
            Token::Number(8.0),
            Token::Number(9.0),
            Token::Rparen,
            Token::Rparen,
        ]
    );
}

#[test]
fn variable_definition() {
    let source = "(define pi (/ 22 7))";
    assert_eq!(
        lexer(source),
        vec![
            Token::Lparen,
            Token::Symbol("define".to_string()),
            Token::Symbol("pi".to_string()),
            Token::Lparen,
            Token::Symbol("/".to_string()),
            Token::Number(22.0),
            Token::Number(7.0),
            Token::Rparen,
            Token::Rparen,
        ]
    );
}
