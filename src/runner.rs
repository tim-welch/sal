use crate::ast::parse;
use crate::interpreter::{evaluate, Value};
use crate::scanner::tokenize;
use std::error::Error;
use std::io;
use std::io::Write;

pub fn run() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::default();
        let res = io::stdin().read_line(&mut line);
        match res {
            Ok(_) => match line.as_str().trim() {
                "quit" => {
                    break;
                }
                _ => match evaluate_line(&line) {
                    Ok(value) => {
                        println!("{:?}", value);
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                },
            },
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}

pub fn evaluate_line(line: &str) -> Result<Value, Box<dyn Error>> {
    let tokens = tokenize(line)?;
    let ast = parse(&tokens)?;
    let value = evaluate(&ast)?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let value = evaluate_line("10 + 2 + 3 * 9 - 4").unwrap();
        assert_eq!(value, Value::Number(35.0));
    }
}
