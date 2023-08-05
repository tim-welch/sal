use crate::ast::Expr;
use std::error::Error;
use std::str::FromStr;

// NOTE: Deriving PartialEq is not going to work in all cases.
// We'll need to use approx_eq! or something to compare f64, but for now this is
// good enough.
#[derive(Debug, PartialEq)]
pub enum Value {
    Number { value: f64 },
}

pub fn evaluate(expr: &Expr) -> Result<Value, Box<dyn Error>> {
    match expr {
        Expr::NumericLiteral { value } => {
            let value = f64::from_str(value)?;
            Ok(Value::Number { value })
        }
        _ => Err("Unsupported expression type".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_number() {
        struct Test {
            expr: Expr,
            expected: Value,
        }
        let tests = vec![
            Test {
                expr: Expr::NumericLiteral {
                    value: String::from("123.345"),
                },
                expected: Value::Number { value: 123.345 },
            },
            Test {
                expr: Expr::NumericLiteral {
                    value: String::from("0"),
                },
                expected: Value::Number { value: 0.0 },
            },
            Test {
                expr: Expr::NumericLiteral {
                    value: String::from("0.0"),
                },
                expected: Value::Number { value: 0.0 },
            },
        ];
        for test in tests {
            let value = evaluate(&test.expr).unwrap();
            assert_eq!(value, test.expected);
        }
    }
}
