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
        let expr = Expr::NumericLiteral {
            value: String::from("123.345"),
        };
        let expected = Value::Number { value: 123.345 };
        let value = evaluate(&expr).unwrap();
        assert_eq!(value, expected);
    }
}
