use liwb::lexer::*;
use liwb::parser::*;

#[test]
fn empty_source_returns_void_object() {
    let source = "";
    assert_eq!(parse(lexer(source)).unwrap()[0], Literal::Void);
}

#[test]
fn basic_arithmetic_operation() {
    let source = "(+ 1 2)";
    assert_eq!(
        parse(lexer(source)).unwrap()[0],
        Literal::List(vec![
            Literal::MathOperator(MathOperators::Add),
            Literal::Number(1.0),
            Literal::Number(2.0),
        ])
    );
}

#[test]
fn nested() {
    let source = "(+ 1 (+ 2 (+ 3 (+ 4 5))))";
    assert_eq!(
        parse(lexer(source)).unwrap()[0],
        Literal::List(vec![
            Literal::MathOperator(MathOperators::Add),
            Literal::Number(1.0),
            Literal::List(vec![
                Literal::MathOperator(MathOperators::Add),
                Literal::Number(2.0),
                Literal::List(vec![
                    Literal::MathOperator(MathOperators::Add),
                    Literal::Number(3.0),
                    Literal::List(vec![
                        Literal::MathOperator(MathOperators::Add),
                        Literal::Number(4.0),
                        Literal::Number(5.0),
                    ])
                ])
            ])
        ])
    );
}

#[test]
fn arithmetic_operation_with_void_on_right() {
    let source = "(+ 1 ())";
    assert_eq!(
        parse(lexer(source)).unwrap()[0],
        Literal::List(vec![
            Literal::MathOperator(MathOperators::Add),
            Literal::Number(1.0),
            Literal::Void
        ])
    );
}

#[test]
fn arithmetic_operation_with_list_on_both_side() {
    let source = "(+ (+ 1 (* 2 3)) (* (/ 4 5) (- 6 7)))";
    assert_eq!(
        parse(lexer(source)).unwrap()[0],
        Literal::List(vec![
            Literal::MathOperator(MathOperators::Add),
            Literal::List(vec![
                Literal::MathOperator(MathOperators::Add),
                Literal::Number(1.0),
                Literal::List(vec![
                    Literal::MathOperator(MathOperators::Multiply),
                    Literal::Number(2.0),
                    Literal::Number(3.0),
                ])
            ]),
            Literal::List(vec![
                Literal::MathOperator(MathOperators::Multiply),
                Literal::List(vec![
                    Literal::MathOperator(MathOperators::Divide),
                    Literal::Number(4.0),
                    Literal::Number(5.0),
                ]),
                Literal::List(vec![
                    Literal::MathOperator(MathOperators::Subtract),
                    Literal::Number(6.0),
                    Literal::Number(7.0),
                ]),
            ]),
        ])
    );
}

#[test]
fn define_variable() {
    let source = "(define pi (/ 22 7))";
    assert_eq!(
        parse(lexer(source)).unwrap()[0],
        Literal::List(vec![
            Literal::Symbol("define".to_string()),
            Literal::Symbol("pi".to_string()),
            Literal::List(vec![
                Literal::MathOperator(MathOperators::Divide),
                Literal::Number(22.0),
                Literal::Number(7.0),
            ])
        ]),
    )
}

#[test]
fn multi_line_statement() {
    let source = "(define pi (/ 22 7))\n(define r 10)\n(define area-of-circle (* pi (* r r)))";
    assert_eq!(
        parse(lexer(source)).unwrap(),
        vec![
            Literal::List(vec![
                Literal::Symbol("define".to_string()),
                Literal::Symbol("pi".to_string()),
                Literal::List(vec![
                    Literal::MathOperator(MathOperators::Divide),
                    Literal::Number(22.0),
                    Literal::Number(7.0),
                ])
            ]),
            Literal::List(vec![
                Literal::Symbol("define".to_string()),
                Literal::Symbol("r".to_string()),
                Literal::Number(10.0)
            ]),
            Literal::List(vec![
                Literal::Symbol("define".to_string()),
                Literal::Symbol("area-of-circle".to_string()),
                Literal::List(vec![
                    Literal::MathOperator(MathOperators::Multiply),
                    Literal::Symbol("pi".to_string()),
                    Literal::List(vec![
                        Literal::MathOperator(MathOperators::Multiply),
                        Literal::Symbol("r".to_string()),
                        Literal::Symbol("r".to_string()),
                    ])
                ])
            ]),
        ]
    );
}

#[test]
fn boolean_variable() {
    let source = "(define x false)";
    assert_eq!(
        parse(lexer(source)).unwrap()[0],
        Literal::List(vec![
            Literal::Symbol("define".to_string()),
            Literal::Symbol("x".to_string()),
            Literal::Boolean(false)
        ])
    );
}

#[test]
fn define_string_variable() {
    let source = "(define message \"Hello, World\")";
    assert_eq!(
        parse(lexer(source)).unwrap()[0],
        Literal::List(vec![
            Literal::Symbol("define".to_string()),
            Literal::Symbol("message".to_string()),
            Literal::String("Hello, World".to_string()),
        ])
    );
}
