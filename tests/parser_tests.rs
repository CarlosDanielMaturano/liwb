use liwb::lexer::*;
use liwb::parser::*;

#[test]
fn empty_source_returns_void_object() {
    let source = "";
    assert_eq!(
        parse(lexer(source)).unwrap(),
        Object::Void
    );
}

#[test]
fn basic_arithmetic_operation() {
    let source = "(+ 1 2)";
    assert_eq!(
        parse(lexer(source)).unwrap(),
        Object::List(vec![
            Object::BinaryOperator(Operator::Add),
            Object::Number(1.0),
            Object::Number(2.0),
        ])
    );
}

#[test]
fn nested() {
    let source = "(+ 1 (+ 2 (+ 3 (+ 4 5))))";
    assert_eq!(
        parse(lexer(source)).unwrap(),
        Object::List(vec![
            Object::BinaryOperator(Operator::Add),
            Object::Number(1.0),
            Object::List(vec![
                Object::BinaryOperator(Operator::Add),
                Object::Number(2.0),
                Object::List(vec![
                    Object::BinaryOperator(Operator::Add),
                    Object::Number(3.0),
                    Object::List(vec![
                        Object::BinaryOperator(Operator::Add),
                        Object::Number(4.0),
                        Object::Number(5.0),
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
        parse(lexer(source)).unwrap(),
        Object::List(vec![
            Object::BinaryOperator(Operator::Add),
            Object::Number(1.0),
            Object::Void
        ])
    );
}

#[test]
fn arithmetic_operation_with_list_on_both_side() {
    let source = "(+ (+ 1 (* 2 3)) (* (/ 4 5) (- 6 7)))";
    assert_eq!(
        parse(lexer(source)).unwrap(),
        Object::List(vec![
            Object::BinaryOperator(Operator::Add),
            Object::List(vec![
                Object::BinaryOperator(Operator::Add),
                Object::Number(1.0),
                Object::List(vec![
                    Object::BinaryOperator(Operator::Multiply),
                    Object::Number(2.0),
                    Object::Number(3.0),
                ])
            ]),
            Object::List(vec![
                Object::BinaryOperator(Operator::Multiply),
                Object::List(vec![
                    Object::BinaryOperator(Operator::Divide),
                    Object::Number(4.0),
                    Object::Number(5.0),
                ]),
                Object::List(vec![
                    Object::BinaryOperator(Operator::Subtract),
                    Object::Number(6.0),
                    Object::Number(7.0),
                ]),
            ]),
        ])
    );
}
