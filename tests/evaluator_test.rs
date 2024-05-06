use liwb::evaluator::eval_from_source;
use liwb::lexer::*;
use liwb::parser::*;

#[test]
fn simple_arithemtic_operation() {
    let literals = parse(lexer("(+ (* 5 (- 5 2)) (/ 12 3))")).unwrap();
    assert_eq!(
        eval_from_source(literals).unwrap(),
        vec![Literal::Number(19.0)]
    );
}

#[test]
fn multi_line_operation() {
    let literals = parse(lexer("(+ 1 1)\n(+ 1 1)\n(+ 1 1)")).unwrap();
    assert_eq!(
        eval_from_source(literals).unwrap(),
        vec![
            Literal::Number(2.0),
            Literal::Number(2.0),
            Literal::Number(2.0),
        ]
    );
}

#[test]
#[should_panic]
fn addition_with_void_should_fail() {
    let literals = parse(lexer("(+ 1 ())")).unwrap();
    let _ = eval_from_source(literals).unwrap();
}

#[test]
fn addition_with_variable() {
    let literals = parse(lexer("(define x 5)\n(+ 1 x)")).unwrap();
    assert_eq!(
        eval_from_source(literals).unwrap(),
        vec![Literal::Void, Literal::Number(6.0),]
    );
}

#[test]
fn multi_variable_definition() {
    let literals = parse(lexer("(define pi (/ 22 7))\n(define r 10)\n(* pi (* r r))")).unwrap();
    assert_eq!(
        eval_from_source(literals).unwrap(),
        vec![
            Literal::Void,
            Literal::Void,
            Literal::Number(314.2857142857143),
        ]
    )
}
