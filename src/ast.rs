use std::error::Error;
use crate::scanner::Token;

type ExprResult = Result<Expr, Box<dyn Error>>;
type Tokens = Vec<Token>;

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Binary{left: Box<Expr>, operator: Token, right: Box<Expr>},
    NumericLiteral{value: String},
}

fn is_eof(tokens: &Tokens, current: usize) -> bool {
    tokens.len() <= current
}

fn literal(tokens: &Tokens, current: usize) -> ExprResult {
    if is_eof(tokens, current) {
        return Err("Unexpected end of file".into());
    }
    match &tokens[current] {
        Token::NumericLiteral { value } => Ok(Expr::NumericLiteral { value: value.to_string() }),
        _ => Err(format!("Unexpected token: {:?}", tokens[current]).into())
    }
}

fn term(tokens: &Tokens, current: usize) -> ExprResult {
    let mut expr = literal(tokens, current)?;
    let mut current: usize = current + 1;

    loop {
        match tokens[current] {
            Token::Plus | Token::Minus => {
                let operator = tokens[current].clone();
                current = current+1;
                let right = literal(tokens, current)?;
                current = current+1;
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
    term(&tokens, 0)
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
}