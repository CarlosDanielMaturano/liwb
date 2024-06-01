use crate::evaluator::{eval_literal, Variables};
use crate::lexer::lexer;
use crate::literals::Literal;
use crate::parser::parser;

use std::collections::HashMap;
use std::io::Write;

const PROMPT_MESSAGE: &'static str = "liwb> ";

pub fn repl() -> ! {
    let mut variables: Variables = HashMap::new();
    let mut deleted_literals: Vec<Literal> = Vec::new();

    let mut eval_from_literals = |literals: Vec<Literal>| -> Result<Vec<Literal>, String> {
        let mut results: Vec<Literal> = Vec::new();
        for literal in literals.into_iter() {
            results.push(eval_literal(
                literal,
                &mut variables,
                &mut deleted_literals,
            )?);
        }
        Ok(results)
    };

    'repl_loop: loop {
        print!("{}", PROMPT_MESSAGE);
        std::io::stdout()
            .flush()
            .expect("Failed to flush the stdout.");

        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read the input.");

        let source = buf.trim();
        let literals = match parser(lexer(source)) {
            Ok(literals) => literals,
            Err(err) => {
                eprintln!("{err}");
                continue 'repl_loop;
            }
        };
        match eval_from_literals(literals) {
            Err(err) => {
                eprintln!("{err}");
                continue 'repl_loop;
            }
            Ok(result) => println!("{:?}", result),
        }
    }
}
