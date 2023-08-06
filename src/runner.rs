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
        struct Test<'a> {
            source: &'a str,
            expected: f64,
        }
        let tests = vec![
            Test {
                source: "10 + 2 + 3 * 9 - 4",
                expected: 35.0,
            },
            Test {
                source: "10 + 2 + 3 * (9 - 4)",
                expected: 27.0,
            },
            Test {
                source: "(10 + 5) * 3",
                expected: 45.0,
            },
            Test {
                source: "(10 * (5-1) - 20) * 3",
                expected: 60.0,
            },
            Test {
                source: "(10 * ((5-1) - (20)))",
                expected: -160.0,
            },
            Test {
                source: "((10 * ((5-1) - (20))) * 3)",
                expected: -480.0,
            },
            Test {
                source: "(((10 * ((5-1) - (20))) * 3))",
                expected: -480.0,
            },
            Test {
                source: "(((10 *\n ((5-1) - (20)))\n * 3))",
                expected: -480.0,
            },
        ];
        for test in tests {
            let value = evaluate_line(test.source).unwrap();
            assert_eq!(value, Value::Number(test.expected));
        }
    }

    #[test]
    fn named_values() {
        let source = "def subtotal = 1 + 2 + 3 + 4;\ndef tax = 0.0425;def total = subtotal * (1 + tax);\ntotal";
        let expected = 10.425;
        let value = evaluate_line(source).unwrap();
        assert_eq!(value, Value::Number(expected));
    }
}
