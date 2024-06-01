use liwb::evaluator::eval_from_literals;
use liwb::lexer::lexer;
use liwb::parser::parser;
use liwb::repl::repl;
use liwb::utils::read_file;

fn main() -> Result<(), String> {
    let Some(file_path) = std::env::args().skip(1).next() else {
        repl();
    };
    let source = read_file(file_path)?;
    let literal = parser(lexer(&source))?;
    let _ = eval_from_literals(literal)?;
    Ok(())
}
