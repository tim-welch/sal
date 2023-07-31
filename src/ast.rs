use std::error::Error;
use crate::scanner::Token;

type ExprResult = Result<Expr, Box<dyn Error>>;
type Tokens = Vec<Token>;

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Binary{left: Box<Expr>, operator: Token, right: Box<Expr>},
    NumericLiteral{value: String},
}

fn is_eos(tokens: &Tokens, current: usize) -> bool {
    tokens.len() <= current
}

fn literal(tokens: &Tokens, current: &mut usize) -> ExprResult {
    if is_eos(tokens, *current) || tokens[*current] == Token::EOF {
        return Err("Unexpected end of file".into());
    }
    match &tokens[*current] {
        Token::NumericLiteral { value } => {
            *current += 1;
            Ok(Expr::NumericLiteral { value: value.to_string() })
        },
        _ => Err(format!("Unexpected token: {:?}", tokens[*current]).into())
    }
}

fn factor(tokens: &Tokens, current: &mut usize) -> ExprResult {
    let mut expr = literal(tokens, current)?;
    loop {
        match tokens[*current] {
            Token::Astrix | Token::Slash => {
                let operator = tokens[*current].clone();
                *current += 1;
                let right = literal(tokens, current)?;
                expr = Expr::Binary { left: Box::new(expr), right: Box::new(right) , operator};
            },
            _ => {
                break;
            }
        }
    } 
    
    Ok(expr)
}

fn term(tokens: &Tokens, current: &mut usize) -> ExprResult {
    let mut expr = factor(tokens, current)?;
    loop {
        match tokens[*current] {
            Token::Plus | Token::Minus => {
                let operator = tokens[*current].clone();
                *current += 1;
                let right = factor(tokens, current)?;
                expr = Expr::Binary { left: Box::new(expr), right: Box::new(right) , operator};
            },
            _ => {
                break;
            }
        }
    } 
    
    Ok(expr)
}

pub fn parse(tokens: &Tokens) -> ExprResult {
    let mut current: usize = 0;
    term(&tokens, &mut current)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_empty() {
        let tokens: Vec<Token> = vec![ ];
        let err = parse(&tokens).unwrap_err();
        assert_eq!(format!("{}", err), String::from("Unexpected end of file"));
    }

    #[test]
    fn parse_number() {
        let tokens: Vec<Token> = vec![
            Token::NumericLiteral { value: "123.345".into() },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, Expr::NumericLiteral{ value: "123.345".into() });
    }
    
    #[test]
    fn addition_is_a_binary_operation() {
        let tokens: Vec<Token> = vec![
            Token::NumericLiteral { value: "123.345".into() },
            Token::Plus,
            Token::NumericLiteral { value: "1.0".into() },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, Expr::Binary{left: Box::new(Expr::NumericLiteral{ value: "123.345".into() }), right: Box::new(Expr::NumericLiteral{ value: "1.0".into() }), operator: Token::Plus});
    }

    #[test]
    fn subtraction_is_a_binary_operation() {
        let tokens: Vec<Token> = vec![
            Token::NumericLiteral { value: "123.345".into() },
            Token::Minus,
            Token::NumericLiteral { value: "1.0".into() },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, Expr::Binary{left: Box::new(Expr::NumericLiteral{ value: "123.345".into() }), right: Box::new(Expr::NumericLiteral{ value: "1.0".into() }), operator: Token::Minus});
    }

    #[test]
    fn addition_subtraction_bind_left_to_right() {
        let tokens: Vec<Token> = vec![
            Token::NumericLiteral { value: "123.345".into() },
            Token::Plus,
            Token::NumericLiteral { value: "1.0".into() },
            Token::Minus,
            Token::NumericLiteral { value: "1.345".into() },
            Token::Plus,
            Token::NumericLiteral { value: "10.0".into() },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, Expr::Binary{
            left: Box::new(Expr::Binary{
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::NumericLiteral{ value: "123.345".into() }),
                    right: Box::new(Expr::NumericLiteral{ value: "1.0".into() }),
                    operator: Token::Plus
                }),
                right: Box::new(Expr::NumericLiteral{ value: "1.345".into() }),
                operator: Token::Minus
            }),
            right: Box::new(Expr::NumericLiteral{ value: "10.0".into() }),
            operator: Token::Plus
        });
    }
    
    #[test]
    fn multiplication_is_a_binary_operation() {
        let tokens: Vec<Token> = vec![
            Token::NumericLiteral { value: "123.345".into() },
            Token::Astrix,
            Token::NumericLiteral { value: "1.0".into() },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, Expr::Binary{left: Box::new(Expr::NumericLiteral{ value: "123.345".into() }), right: Box::new(Expr::NumericLiteral{ value: "1.0".into() }), operator: Token::Astrix});
    }

    #[test]
    fn division_is_a_binary_operation() {
        let tokens: Vec<Token> = vec![
            Token::NumericLiteral { value: "123.345".into() },
            Token::Slash,
            Token::NumericLiteral { value: "1.0".into() },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, Expr::Binary{left: Box::new(Expr::NumericLiteral{ value: "123.345".into() }), right: Box::new(Expr::NumericLiteral{ value: "1.0".into() }), operator: Token::Slash});
    }

    #[test]
    fn multiplication_division_bind_left_to_right() {
        let tokens: Vec<Token> = vec![
            Token::NumericLiteral { value: "123.345".into() },
            Token::Astrix,
            Token::NumericLiteral { value: "1.0".into() },
            Token::Slash,
            Token::NumericLiteral { value: "1.345".into() },
            Token::Astrix,
            Token::NumericLiteral { value: "10.0".into() },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, Expr::Binary{
            left: Box::new(Expr::Binary{
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::NumericLiteral{ value: "123.345".into() }),
                    right: Box::new(Expr::NumericLiteral{ value: "1.0".into() }),
                    operator: Token::Astrix
                }),
                right: Box::new(Expr::NumericLiteral{ value: "1.345".into() }),
                operator: Token::Slash
            }),
            right: Box::new(Expr::NumericLiteral{ value: "10.0".into() }),
            operator: Token::Astrix
        });
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
            Token::NumericLiteral { value: "123.345".into() },
            Token::Plus,
            Token::NumericLiteral { value: "1.0".into() },
            Token::Slash,
            Token::NumericLiteral { value: "1.345".into() },
            Token::Minus,
            Token::NumericLiteral { value: "10.0".into() },
            Token::EOF,
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, Expr::Binary{
            left: Box::new(Expr::Binary { 
                left: Box::new(Expr::NumericLiteral { value: "123.345".into() }),
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::NumericLiteral { value: "1.0".into() }),
                    right: Box::new(Expr::NumericLiteral { value: "1.345".into()}),
                    operator: Token::Slash,
                }),
                operator: Token::Plus, 
             }),
            right: Box::new(Expr::NumericLiteral { value: "10.0".into() }),
            operator: Token::Minus,
        });
    }
    
}