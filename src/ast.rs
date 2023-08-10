use crate::scanner::Token;
use std::error::Error;

// TODO: Use recursion to remove mutability

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
    NamedValue { identifier: Token, expr: Box<Expr> },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    NumericLiteral {
        value: String,
    },
}

struct ExprInfo {
    expr: Expr,
    used: usize,
}

struct Program {
    statements: Vec<Stmt>,
    expr: Expr,
}

type ExprResult = Result<ExprInfo, Box<dyn Error>>;
type Tokens = Vec<Token>;

fn is_eos(tokens: &Tokens, current: usize) -> bool {
    tokens.len() <= current || tokens[current] == Token::EOF
}

pub fn parse(tokens: &Tokens) -> Result<Program, Box<dyn Error>> {
    let mut statements: Vec<Stmt> = vec![];
    let mut used: usize = 0;
    while !is_eos(tokens, used) {
        let statement_info = statement(tokens, used)?;
        if let Some(statement_info) = statement_info {
            statements.push(statement_info.0);
            used += statement_info.1;
        } else {
            break;
        }
    }

    let root = expression(tokens, used);
    match root {
        Ok(root) => Ok(Program {
            statements,
            expr: root.expr,
        }),
        Err(err) => Err(err),
    }
}

fn statement(tokens: &Tokens, current: usize) -> Result<Option<(Stmt, usize)>, Box<dyn Error>> {
    match tokens[current] {
        Token::Def => named_value_definition(tokens, current),
        _ => Ok(None),
    }
}

fn named_value_definition(
    tokens: &Tokens,
    current: usize,
) -> Result<Option<(Stmt, usize)>, Box<dyn Error>> {
    let mut used: usize = 0;
    match &tokens[current + used] {
        Token::Def => {
            used += 1;
            match &tokens[current + used] {
                Token::Identifier { value: _ } => {
                    let identifier = &tokens[current + used];
                    used += 1;
                    match &tokens[current + used] {
                        Token::Equal => {
                            used += 1;
                            let expr = expression(tokens, current + used)?;
                            used += expr.used;
                            match &tokens[current + used] {
                                Token::SemiColon => Ok(Some((
                                    Stmt::NamedValue {
                                        identifier: identifier.clone(),
                                        expr: Box::new(expr.expr),
                                    },
                                    used + 1,
                                ))),
                                _ => Err(format!(
                                    "Expected a ; but found: {:?}",
                                    tokens[current + used]
                                )
                                .into()),
                            }
                        }
                        _ => Err(
                            format!("Expected an = but found: {:?}", tokens[current + used]).into(),
                        ),
                    }
                }
                _ => Err(format!(
                    "Expected an identifier but found: {:?}",
                    tokens[current + used]
                )
                .into()),
            }
        }
        _ => panic!("Unreachable"),
    }
}

fn expression(tokens: &Tokens, current: usize) -> ExprResult {
    term(tokens, current)
}

fn term(tokens: &Tokens, current: usize) -> ExprResult {
    let fact = factor(tokens, current)?;
    let mut expr = fact.expr;
    let mut used = fact.used;

    while !is_eos(tokens, current + used) {
        match tokens[current + used] {
            Token::Plus | Token::Minus => {
                let operator = tokens[current + used].clone();
                used += 1;
                let fact = factor(tokens, current + used)?;
                let right = fact.expr;
                used += fact.used;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    right: Box::new(right),
                    operator,
                };
            }
            _ => {
                break;
            }
        }
    }

    Ok(ExprInfo { expr, used })
}

fn factor(tokens: &Tokens, current: usize) -> ExprResult {
    let lit = primary(tokens, current)?;
    let mut expr = lit.expr;
    let mut used: usize = lit.used;
    while !is_eos(tokens, current + used) {
        match tokens[current + used] {
            Token::Astrix | Token::Slash => {
                let operator = tokens[current + used].clone();
                used += 1;
                let lit = primary(tokens, current + used)?;
                let right = lit.expr;
                used += lit.used;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    right: Box::new(right),
                    operator,
                };
            }
            _ => {
                break;
            }
        }
    }

    Ok(ExprInfo { expr, used })
}

fn primary(tokens: &Tokens, current: usize) -> ExprResult {
    if is_eos(tokens, current) {
        return Err("Unexpected end of file".into());
    }

    match tokens[current] {
        Token::NumericLiteral { .. } => literal(&tokens[current]),
        Token::OpenParen => {
            let mut used: usize = 1;
            let expr = expression(tokens, current + used)?;
            used += expr.used;
            let expr = expr.expr;
            match tokens[current + used] {
                Token::CloseParen => Ok(ExprInfo {
                    expr: Expr::Grouping {
                        expr: Box::new(expr),
                    },
                    used: used + 1,
                }),
                _ => Err(format!(
                    "Expected to find Close Parentheses, but found: {:?}",
                    tokens[current + used]
                )
                .into()),
            }
        }
        _ => Err(format!("Unexpected token: {:?}", tokens[current]).into()),
    }
}

fn literal(token: &Token) -> ExprResult {
    match token {
        Token::NumericLiteral { value } => Ok(ExprInfo {
            expr: Expr::NumericLiteral {
                value: value.to_string(),
            },
            used: 1,
        }),
        _ => Err(format!("Token not a literal: {:?}", token).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::tokenize;

    #[test]
    fn parse_empty() {
        let tokens: Vec<Token> = vec![];
        let err = parse(&tokens).unwrap_err();
        assert_eq!(format!("{}", err), String::from("Unexpected end of file"));
    }

    #[test]
    fn parse_number() {
        let tokens: Vec<Token> = vec![
            Token::NumericLiteral {
                value: "123.345".into(),
            },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(
            ast,
            Expr::NumericLiteral {
                value: "123.345".into()
            }
        );
    }

    #[test]
    fn addition_is_a_binary_operation() {
        let tokens: Vec<Token> = vec![
            Token::NumericLiteral {
                value: "123.345".into(),
            },
            Token::Plus,
            Token::NumericLiteral {
                value: "1.0".into(),
            },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::NumericLiteral {
                    value: "123.345".into()
                }),
                right: Box::new(Expr::NumericLiteral {
                    value: "1.0".into()
                }),
                operator: Token::Plus
            }
        );
    }

    #[test]
    fn subtraction_is_a_binary_operation() {
        let tokens: Vec<Token> = vec![
            Token::NumericLiteral {
                value: "123.345".into(),
            },
            Token::Minus,
            Token::NumericLiteral {
                value: "1.0".into(),
            },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::NumericLiteral {
                    value: "123.345".into()
                }),
                right: Box::new(Expr::NumericLiteral {
                    value: "1.0".into()
                }),
                operator: Token::Minus
            }
        );
    }

    #[test]
    fn addition_subtraction_bind_left_to_right() {
        let tokens: Vec<Token> = vec![
            Token::NumericLiteral {
                value: "123.345".into(),
            },
            Token::Plus,
            Token::NumericLiteral {
                value: "1.0".into(),
            },
            Token::Minus,
            Token::NumericLiteral {
                value: "1.345".into(),
            },
            Token::Plus,
            Token::NumericLiteral {
                value: "10.0".into(),
            },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::NumericLiteral {
                            value: "123.345".into()
                        }),
                        right: Box::new(Expr::NumericLiteral {
                            value: "1.0".into()
                        }),
                        operator: Token::Plus
                    }),
                    right: Box::new(Expr::NumericLiteral {
                        value: "1.345".into()
                    }),
                    operator: Token::Minus
                }),
                right: Box::new(Expr::NumericLiteral {
                    value: "10.0".into()
                }),
                operator: Token::Plus
            }
        );
    }

    #[test]
    fn multiplication_is_a_binary_operation() {
        let tokens: Vec<Token> = vec![
            Token::NumericLiteral {
                value: "123.345".into(),
            },
            Token::Astrix,
            Token::NumericLiteral {
                value: "1.0".into(),
            },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::NumericLiteral {
                    value: "123.345".into()
                }),
                right: Box::new(Expr::NumericLiteral {
                    value: "1.0".into()
                }),
                operator: Token::Astrix
            }
        );
    }

    #[test]
    fn division_is_a_binary_operation() {
        let tokens: Vec<Token> = vec![
            Token::NumericLiteral {
                value: "123.345".into(),
            },
            Token::Slash,
            Token::NumericLiteral {
                value: "1.0".into(),
            },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::NumericLiteral {
                    value: "123.345".into()
                }),
                right: Box::new(Expr::NumericLiteral {
                    value: "1.0".into()
                }),
                operator: Token::Slash
            }
        );
    }

    #[test]
    fn multiplication_division_bind_left_to_right() {
        let tokens: Vec<Token> = vec![
            Token::NumericLiteral {
                value: "123.345".into(),
            },
            Token::Astrix,
            Token::NumericLiteral {
                value: "1.0".into(),
            },
            Token::Slash,
            Token::NumericLiteral {
                value: "1.345".into(),
            },
            Token::Astrix,
            Token::NumericLiteral {
                value: "10.0".into(),
            },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::NumericLiteral {
                            value: "123.345".into()
                        }),
                        right: Box::new(Expr::NumericLiteral {
                            value: "1.0".into()
                        }),
                        operator: Token::Astrix
                    }),
                    right: Box::new(Expr::NumericLiteral {
                        value: "1.345".into()
                    }),
                    operator: Token::Slash
                }),
                right: Box::new(Expr::NumericLiteral {
                    value: "10.0".into()
                }),
                operator: Token::Astrix
            }
        );
    }

    #[test]
    fn multiplication_has_precedence_over_addition() {
        // 123.345 + 1.0 / 1.345 - 10.0
        //             -
        //           /   \
        //         +     10.0
        //        /  \
        // 123.345   /
        //         /  \
        //      1.0    1.345

        let tokens: Vec<Token> = vec![
            Token::NumericLiteral {
                value: "123.345".into(),
            },
            Token::Plus,
            Token::NumericLiteral {
                value: "1.0".into(),
            },
            Token::Slash,
            Token::NumericLiteral {
                value: "1.345".into(),
            },
            Token::Minus,
            Token::NumericLiteral {
                value: "10.0".into(),
            },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::NumericLiteral {
                        value: "123.345".into()
                    }),
                    right: Box::new(Expr::Binary {
                        left: Box::new(Expr::NumericLiteral {
                            value: "1.0".into()
                        }),
                        right: Box::new(Expr::NumericLiteral {
                            value: "1.345".into()
                        }),
                        operator: Token::Slash,
                    }),
                    operator: Token::Plus,
                }),
                right: Box::new(Expr::NumericLiteral {
                    value: "10.0".into()
                }),
                operator: Token::Minus,
            }
        );
    }

    #[test]
    fn integrates_with_scanner() {
        let tokens = tokenize("10 + 11").unwrap();
        let ast = parse(&tokens).unwrap();
        println!("{:?}", ast);
    }

    #[test]
    fn named_value_definitions() {
        let tokens: Vec<Token> = vec![
            Token::Def,
            Token::Identifier {
                value: "subtotal".into(),
            },
            Token::Equal,
            Token::NumericLiteral { value: "1".into() },
            Token::SemiColon,
            Token::NumericLiteral { value: "10".into() },
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, Expr::NumericLiteral { value: "10".into() },);
    }
}
