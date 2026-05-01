use std::{
    env,
    process,
};
use org::cli::parser::CliParser;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut parser = CliParser::new();

    if let Err(e) = parser.read_args(args) {
	eprintln!("Problem parsing arguments: {}", e);
	process::exit(1);
    }

    if let Err(e) = org::run(parser) {
	eprintln!("Program error: {}", e);
	process::exit(2);
    }
}
