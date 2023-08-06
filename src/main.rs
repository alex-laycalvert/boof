mod interpreter;
mod error;
mod expr;
mod lexer;
mod parser;
mod token;

use error::Error;
use lexer::Lexer;
use parser::Parser;
use std::env::args;
use std::path::Path;
use std::{
    fs,
    io::{self, Write},
    process,
};

fn main() {
    let args: Vec<String> = args().collect();

    let result = match args.len() {
        1 => run_prompt(),
        2 => run_file(Path::new(&args[1])),
        _ => Err(Error::usage()),
    };

    if let Err(e) = result {
        eprintln!("{}", e.message);
        process::exit(e.code);
    }
}

fn run_file(path: &Path) -> Result<(), Error> {
    let data = fs::read_to_string(path).or_else(|_| {
        Err(Error {
            code: 1,
            message: format!("Failed to read file {path:?}"),
        })
    })?;
    run(data)
}

fn run_prompt() -> Result<(), Error> {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).or_else(|_| {
            Err(Error {
                code: 1,
                message: "Failed to read line".to_string(),
            })
        })?;
        if bytes_read == 0 {
            break;
        }
        run(line)?;
    }
    Ok(())
}

fn run(source: String) -> Result<(), Error> {
    let mut lexer = Lexer::from(source);
    let tokens = lexer.scan()?;
    let mut parser = Parser::from(tokens);
    let expression = parser.parse();
    match expression {
        Ok(expr) => expr.print(),
        Err(err) => eprintln!("{}", err.message),
    }
    Ok(())
}
