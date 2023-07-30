use std::error::Error;

type Source = Vec<char>;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    NumericLiteral,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenType,
    value: String,
}

struct TokenInfo {
    token: Token,
    used: usize,
}

fn is_end(source: &Source, current: usize) -> bool {
    source.len() <= current
}

fn number(source: &Source, current: usize) -> TokenInfo {
    let mut used = 0;
    while !is_end(source, current+used) && source[current+used].is_ascii_digit() {
        used += 1;
    }
    if !is_end(source, current+used) && source[current+used] == '.' {
        used += 1;
    }
    while !is_end(source, current+used) && source[current+used].is_ascii_digit() {
        used += 1;
    }
    
    TokenInfo {token: Token{kind: TokenType::NumericLiteral, value: source[current..current+used].iter().collect()}, used}
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let source: Source = source.chars().collect();
    let mut tokens = vec![];
    let mut current: usize = 0;

    while !is_end(&source, current) {
        match source[current] {
            '0'..='9' => {
                let token_info = number(&source, current);
                tokens.push(token_info.token);
                current += token_info.used;
            }
            _ => {
                return Err("Unknown token".into());
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_returns_no_tokens() {
        let tokens = tokenize("").unwrap();
        assert_eq!(tokens.len(), 0);
    }
    
    #[test]
    fn tokenize_numeric_literal() {
        struct Test {
            source: &'static str,
            expected: Token,
        }
        let tests = [
            Test { source: "123.456", expected: Token{ kind: TokenType::NumericLiteral, value: "123.456".into()}},
            Test { source: "1", expected: Token{ kind: TokenType::NumericLiteral, value: "1".into()}},
            Test { source: "0", expected: Token{ kind: TokenType::NumericLiteral, value: "0".into()}},
            Test { source: "1234567890", expected: Token{ kind: TokenType::NumericLiteral, value: "1234567890".into()}},
            Test { source: "0.123456789", expected: Token{ kind: TokenType::NumericLiteral, value: "0.123456789".into()}},
        ];
        for test in tests {
            let tokens = tokenize(test.source).unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0], test.expected);
            assert_eq!(tokens[0].kind, TokenType::NumericLiteral);

        }
    }
}
