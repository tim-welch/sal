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
    tokens.len() > current
}

fn literal(tokens: &Tokens, current: usize) -> ExprResult {
    match &tokens[current] {
        Token::NumericLiteral { value } => Ok(Expr::NumericLiteral { value: value.to_string() }),
        _ => Err(format!("Unexpected token: {:?}", tokens[current]).into())
    }
    
}

pub fn parse(tokens: &Tokens) -> ExprResult {
    literal(&tokens, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        let tokens: Vec<Token> = vec![
            Token::NumericLiteral { value: "123.345".into() },
        ];
        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, Expr::NumericLiteral{ value: "123.345".into() });
    }
}