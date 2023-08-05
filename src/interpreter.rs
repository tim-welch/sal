use crate::ast::Expr;
use crate::scanner::Token;
use std::error::Error;
use std::str::FromStr;

// NOTE: Deriving PartialEq is not going to work in all cases.
// We'll need to use approx_eq! or something to compare f64, but for now this is
// good enough.
#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
}

pub fn evaluate(expr: &Expr) -> Result<Value, Box<dyn Error>> {
    match expr {
        Expr::NumericLiteral { value } => {
            let value = f64::from_str(value)?;
            Ok(Value::Number(value))
        }
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let left = evaluate(left)?;
            let right = evaluate(right)?;
            match (operator, left, right) {
                (Token::Plus, Value::Number(left), Value::Number(right)) => {
                    Ok(Value::Number(left + right))
                }
                (Token::Minus, Value::Number(left), Value::Number(right)) => {
                    Ok(Value::Number(left - right))
                }
                (Token::Astrix, Value::Number(left), Value::Number(right)) => {
                    Ok(Value::Number(left * right))
                }
                (Token::Slash, Value::Number(left), Value::Number(right)) => {
                    Ok(Value::Number(left / right))
                }
                _ => Err("Not supported".into()),
            }
        }
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
                expected: Value::Number(123.345),
            },
            Test {
                expr: Expr::NumericLiteral {
                    value: String::from("0"),
                },
                expected: Value::Number(0.0),
            },
            Test {
                expr: Expr::NumericLiteral {
                    value: String::from("0.0"),
                },
                expected: Value::Number(0.0),
            },
        ];
        for test in tests {
            let value = evaluate(&test.expr).unwrap();
            assert_eq!(value, test.expected);
        }
    }

    #[test]
    fn evaluate_addition() {
        struct Test {
            expr: Expr,
            expected: Value,
        }
        let tests = vec![
            Test {
                expr: Expr::Binary {
                    left: Box::new(Expr::NumericLiteral {
                        value: "123.345".into(),
                    }),
                    right: Box::new(Expr::NumericLiteral {
                        value: "1.0".into(),
                    }),
                    operator: Token::Plus,
                },
                expected: Value::Number(124.345),
            },
            Test {
                expr: Expr::Binary {
                    left: Box::new(Expr::NumericLiteral {
                        value: "8753.0".into(),
                    }),
                    right: Box::new(Expr::NumericLiteral {
                        value: "0.0".into(),
                    }),
                    operator: Token::Plus,
                },
                expected: Value::Number(8753.0),
            },
        ];
        for test in tests {
            let value = evaluate(&test.expr).unwrap();
            assert_eq!(value, test.expected);
        }
    }

    #[test]
    fn evaluate_subtraction() {
        struct Test {
            expr: Expr,
            expected: Value,
        }
        let tests = vec![
            Test {
                expr: Expr::Binary {
                    left: Box::new(Expr::NumericLiteral {
                        value: "123.345".into(),
                    }),
                    right: Box::new(Expr::NumericLiteral {
                        value: "1.0".into(),
                    }),
                    operator: Token::Minus,
                },
                expected: Value::Number(122.345),
            },
            Test {
                expr: Expr::Binary {
                    left: Box::new(Expr::NumericLiteral {
                        value: "8753.0".into(),
                    }),
                    right: Box::new(Expr::NumericLiteral {
                        value: "0.0".into(),
                    }),
                    operator: Token::Minus,
                },
                expected: Value::Number(8753.0),
            },
        ];
        for test in tests {
            let value = evaluate(&test.expr).unwrap();
            assert_eq!(value, test.expected);
        }
    }

    #[test]
    fn evaluate_multiplication() {
        struct Test {
            expr: Expr,
            expected: Value,
        }
        let tests = vec![
            Test {
                expr: Expr::Binary {
                    left: Box::new(Expr::NumericLiteral {
                        value: "123.345".into(),
                    }),
                    right: Box::new(Expr::NumericLiteral {
                        value: "1.0".into(),
                    }),
                    operator: Token::Astrix,
                },
                expected: Value::Number(123.345),
            },
            Test {
                expr: Expr::Binary {
                    left: Box::new(Expr::NumericLiteral {
                        value: "8753.0".into(),
                    }),
                    right: Box::new(Expr::NumericLiteral {
                        value: "0.0".into(),
                    }),
                    operator: Token::Astrix,
                },
                expected: Value::Number(0.0),
            },
        ];
        for test in tests {
            let value = evaluate(&test.expr).unwrap();
            assert_eq!(value, test.expected);
        }
    }

    #[test]
    fn evaluate_division() {
        struct Test {
            expr: Expr,
            expected: Value,
        }
        let tests = vec![
            Test {
                expr: Expr::Binary {
                    left: Box::new(Expr::NumericLiteral {
                        value: "123.345".into(),
                    }),
                    right: Box::new(Expr::NumericLiteral {
                        value: "1.0".into(),
                    }),
                    operator: Token::Slash,
                },
                expected: Value::Number(123.345),
            },
            // TODO: Fix floating point rounding error
            // Test {
            //     expr: Expr::Binary {
            //         left: Box::new(Expr::NumericLiteral {
            //             value: "8753.0".into(),
            //         }),
            //         right: Box::new(Expr::NumericLiteral {
            //             value: "2.2".into(),
            //         }),
            //         operator: Token::Slash,
            //     },
            //     expected: Value::Number(3978.636363636363),
            // },
        ];
        for test in tests {
            let value = evaluate(&test.expr).unwrap();
            assert_eq!(value, test.expected);
        }
    }
}
