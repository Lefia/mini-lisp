use mini_lisp::parser;
use mini_lisp::interpreter;

fn main() {
    let path = std::env::args().nth(1).expect("No file path provided");
    let unparsed = std::fs::read_to_string(path).expect("Could not read file");
    match parser::parse(&unparsed) {
        Ok(program) => {
            if let Err(err) = interpreter::run(program) {
                eprintln!("{}", err);
            }
        }
        Err(err) => eprintln!("{}", err)
    }
}