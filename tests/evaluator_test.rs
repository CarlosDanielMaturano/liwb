use liwb::evaluator::eval_from_literals;
use liwb::lexer::*;
use liwb::literals::*;
use liwb::parser::*;
use liwb::utils::*;

#[test]
fn simple_arithemtic_operation() {
    let literals = parse(lexer("(+ (* 5 (- 5 2)) (/ 12 3))")).unwrap();
    assert_eq!(
        eval_from_literals(literals).unwrap(),
        vec![Literal::Number(19.0)]
    );
}

#[test]
fn multi_line_operation() {
    let literals = parse(lexer("(+ 1 1)\n(+ 1 1)\n(+ 1 1)")).unwrap();
    assert_eq!(
        eval_from_literals(literals).unwrap(),
        vec![
            Literal::Number(2.0),
            Literal::Number(2.0),
            Literal::Number(2.0),
        ]
    );
}

#[test]
fn arithemtic_operation_with_multiple_arguments() {
    let literals = parse(lexer("(+ 1 2 3 4 5 6 7 8 9 10)")).unwrap();
    assert_eq!(
        eval_from_literals(literals).unwrap(),
        vec![Literal::Number(55.0),]
    );
    let literals = parse(lexer("(* 1 2 3 4 5 6 7 8 9 10)")).unwrap();
    assert_eq!(
        eval_from_literals(literals).unwrap(),
        vec![Literal::Number(3628800.0)]
    );
    let literals = parse(lexer("(/ 1 2 3 4 5 6 7 8 9 10)")).unwrap();
    assert_eq!(
        eval_from_literals(literals).unwrap(),
        vec![Literal::Number(2.7557319223985894e-7)]
    );
    let literals = parse(lexer("(- 1 2 3 4 5 6 7 8 9 10)")).unwrap();
    assert_eq!(
        eval_from_literals(literals).unwrap(),
        vec![Literal::Number(-53.0)]
    );
}

#[test]
#[should_panic]
fn addition_with_void_should_fail() {
    let literals = parse(lexer("(+ 1 ())")).unwrap();
    let _ = eval_from_literals(literals).unwrap();
}

#[test]
fn addition_with_variable() {
    let literals = parse(lexer("(define x 5)\n(+ 1 x)")).unwrap();
    assert_eq!(
        eval_from_literals(literals).unwrap(),
        vec![Literal::Void, Literal::Number(6.0),]
    );
}

#[test]
fn equal_operator() {
    let literals = parse(lexer("(define 5-is-equal-10 (= 5 10))\n(5-is-equal-10)")).unwrap();
    assert_eq!(
        eval_from_literals(literals).unwrap(),
        vec![Literal::Void, Literal::Boolean(false),]
    );
    let literals = parse(lexer("(define 5-is-equal-5 (= 5 5))\n(5-is-equal-5)")).unwrap();
    assert_eq!(
        eval_from_literals(literals).unwrap(),
        vec![Literal::Void, Literal::Boolean(true),]
    );
}

#[test]
fn multi_variable_definition() {
    let literals = parse(lexer("(define pi (/ 22 7))\n(define r 10)\n(* pi (* r r))")).unwrap();
    assert_eq!(
        eval_from_literals(literals).unwrap(),
        vec![
            Literal::Void,
            Literal::Void,
            Literal::Number(314.2857142857143),
        ]
    )
}

#[test]
fn area_of_trapezium() {
    let source = read_file("liwb/area_of_trapezium.liwb").unwrap();
    let literals = parse(lexer(&source)).unwrap();
    assert_eq!(
        eval_from_literals(literals).unwrap().pop().unwrap(),
        Literal::Number(30.0),
    );
}

#[test]
fn calculate_hypotenuses() {
    let source = read_file("liwb/calculate_hypotenuses.liwb").unwrap();
    let literals = parse(lexer(&source)).unwrap();
    assert_eq!(
        eval_from_literals(literals).unwrap().pop().unwrap(),
        Literal::Number(5.0),
    );
}

#[test]
fn calculate_quadratic_function() {
    let source = read_file("liwb/calculate_quadratic_function.liwb").unwrap();
    let literals = parse(lexer(&source)).unwrap();
    assert_eq!(
        eval_from_literals(literals)
            .unwrap()
            .into_iter()
            .rev()
            .take(2)
            .rev()
            .collect::<Vec<_>>(),
        vec![Literal::Number(1.0), Literal::Number(-3.0),],
    );
}

#[test]
fn if_statement() {
    let source = read_file("liwb/if_statement.liwb").unwrap();
    let literals = parse(lexer(&source)).unwrap();
    assert_eq!(
        eval_from_literals(literals)
            .unwrap()
            .into_iter()
            .rev()
            .take(2)
            .rev()
            .collect::<Vec<_>>(),
        vec![Literal::Number(10.0), Literal::Number(0.0)],
    );
}

#[test]
fn string_variable() {
    let literals = parse(lexer("(define name \"Daniel\") (name)")).unwrap();
    assert_eq!(
        eval_from_literals(literals).unwrap(),
        vec![Literal::Void, Literal::String("Daniel".to_string())]
    );
}

#[test]
fn vector_variable() {
    let literals = parse(lexer(
        r#"(define numbers [1 "two" 3 "four" (+ 4 1)]) (numbers)"#,
    ))
    .unwrap();
    assert_eq!(
        eval_from_literals(literals).unwrap(),
        vec![
            Literal::Void,
            Literal::Vector(vec![
                Literal::Number(1.0),
                Literal::String("two".to_string()),
                Literal::Number(3.0),
                Literal::String("four".to_string()),
                Literal::List(vec![
                    Literal::MathOperator(MathOperators::Add),
                    Literal::Number(4.0),
                    Literal::Number(1.0),
                ])
            ])
        ]
    );
}

#[test]
fn get_element_of_vector() {
    let source = read_file("liwb/get_element_of_vector.liwb").unwrap();
    let literals = parse(lexer(&source)).unwrap();
    assert_eq!(
        eval_from_literals(literals)
            .unwrap()
            .into_iter()
            .rev()
            .take(1)
            .collect::<Vec<_>>(),
        vec![Literal::Number(1.0)],
    );
}

#[test]
fn out_of_bounds_vector_index() {
    let source = read_file("liwb/out_of_bounds_vector_index.liwb").unwrap();
    let literals = parse(lexer(&source)).unwrap();
    assert_eq!(
        eval_from_literals(literals)
            .unwrap()
            .into_iter()
            .rev()
            .take(1)
            .collect::<Vec<_>>(),
        vec![Literal::Void],
    );
}

#[test]
fn functions() {
    let source = read_file("liwb/functions.liwb").unwrap();
    let literals = parse(lexer(&source)).unwrap();
    assert_eq!(
        eval_from_literals(literals)
            .unwrap()
            .into_iter()
            .rev()
            .take(4)
            .rev()
            .collect::<Vec<_>>(),
        vec![
            Literal::Number(-140.0),
            Literal::String("Name".to_string()),
            Literal::Number(0.0),
            Literal::Number(10.0),
        ],
    );
}

#[test]
fn fibonnaci_function() {
    let source = read_file("liwb/fibonnaci_function.liwb").unwrap();
    let literals = parse(lexer(&source)).unwrap();
    assert_eq!(
        eval_from_literals(literals)
            .unwrap()
            .into_iter()
            .rev()
            .take(1)
            .collect::<Vec<_>>(),
        vec![Literal::Number(3.0),],
    );
}

#[test]
fn relational_operators() {
    let source = read_file("liwb/relational_operators.liwb").unwrap();
    let literals = parse(lexer(&source)).unwrap();
    assert_eq!(
        eval_from_literals(literals)
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        vec![
            Literal::Boolean(true),
            Literal::Boolean(false),
            Literal::Boolean(true),
            Literal::Boolean(false),
        ],
    );
}

#[test]
fn vectors() {
    let source = read_file("liwb/vectors.liwb").unwrap();
    let literals = parse(lexer(&source)).unwrap();
    assert_eq!(
        eval_from_literals(literals)
            .unwrap()
            .into_iter()
            .rev()
            .take(6)
            .collect::<Vec<_>>(),
        vec![
            Literal::Vector((1..=10).map(|n| Literal::Number(n as f64)).collect()),
            Literal::Vector(vec![
                Literal::Number(1.0),
                Literal::Number(3.0),
                Literal::Number(5.0),
                Literal::Vector(vec![
                    Literal::Number(2.0),
                    Literal::Number(4.0),
                    Literal::Number(6.0),
                ])
            ]),
            Literal::Number(2.0),
            Literal::Void,
            Literal::Number(4.0),
            Literal::Number(5.0),
        ],
    );
}

#[test]
fn fibonnaci_vector() {
    let source = read_file("liwb/fibonnaci_vector.liwb").unwrap();
    let literals = parse(lexer(&source)).unwrap();
    assert_eq!(
        eval_from_literals(literals)
            .unwrap()
            .into_iter()
            .rev()
            .take(1)
            .collect::<Vec<_>>(),
        vec![Literal::Vector(vec![
            Literal::Number(0.0),
            Literal::Number(1.0),
            Literal::Number(1.0),
            Literal::Number(2.0),
            Literal::Number(3.0),
            Literal::Number(5.0),
            Literal::Number(8.0),
            Literal::Number(13.0),
            Literal::Number(21.0),
            Literal::Number(34.0),
            Literal::Number(55.0),
            Literal::Number(89.0),
            Literal::Number(144.0)
        ])],
    );
}

#[test]
#[should_panic]
fn invalid_map() {
    let source = read_file("liwb/invalid_map.liwb").unwrap();
    let literals = parse(lexer(&source)).unwrap();
    eval_from_literals(literals).unwrap();
}
