mod args;
mod parser;

use args::Args;
use parser::Parser;

fn main() {
    let args = Args::new();
    let parser = Parser::new(args);
    let output = parser.get_output();

    print!("{}", output);
}
