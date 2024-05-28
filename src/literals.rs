use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum MathOperators {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Operator {
    Equal,
    LessThan,
    BiggerThan,
    LessOrEqualThan,
    BiggerOrEqualThan,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
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

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Literal::Number(n) => n.to_string(),
                Literal::Boolean(b) => b.to_string(),
                Literal::String(s) | Literal::Symbol(s) => s.to_string(),
                Literal::Vector(v) => {
                    let mut result = String::from("[ ");
                    v.into_iter().for_each(|literal| {
                        result += &format!("{} ", literal.to_string());
                    });
                    result += " ]";
                    result
                },
                Literal::Function { name, .. } => format!("(liwb function#{name})"),
                Literal::Void | _ => "()".to_string(),
            }
        )
    }
}
