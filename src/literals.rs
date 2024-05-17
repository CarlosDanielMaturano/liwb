#[derive(Debug, PartialEq, Clone)]
pub enum MathOperators {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Equal,
    LessThan,
    BiggerThan,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Void,
    List(Vec<Literal>),
    Vector(Vec<Literal>),
    Number(f64),
    Symbol(String),
    String(String),
    MathOperator(MathOperators),
    Boolean(bool),
    BinaryOperator(Operator),
    If,
    Function {
        name: String,
        args: Vec<String>,
        body: Box<Literal>,
    },
}
