use liwb::utils::read_file;
use liwb::lexer::lexer;
use liwb::parser::parser;
use liwb::evaluator::eval_from_literals;

fn main() -> Result<(), String> {
    let Some(file_path) = std::env::args().skip(1).next() else {
       return Err(format!("Error. Missing file_path"))
    };
    dbg!(&file_path);
    let source = read_file(file_path)?;
    let literal = parser(lexer(&source))?;
    let _ = eval_from_literals(literal)?;
    Ok(())
}
