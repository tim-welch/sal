use std::error::Error;

type Source = Vec<char>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token{
    EOF,
    
    // Literals
    NumericLiteral{value: String},
    
    // Operators
    Plus,
    Minus,
    Astrix,
    Slash,
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
    
    TokenInfo {token: Token::NumericLiteral{value: source[current..current+used].iter().collect()}, used}
}

fn eat_whitespace(source: &Source, current: usize) -> usize {
    let mut used: usize = 0;
    while !is_end(source, current+used) && source[current+used].is_whitespace() {
        used += 1;
    }
    used
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
            '+' => {
                tokens.push(Token::Plus);
                current += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                current += 1;
            }
            '*' => {
                tokens.push(Token::Astrix);
                current += 1;
            }
            '/' => {
                tokens.push(Token::Slash);
                current += 1;
            }
            _ => {
                let used = eat_whitespace(&source,current);
                if used > 0 {
                    current += used;
                } else {
                    return Err("Unknown token".into());
                }
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
            Test { source: "123.456", expected: Token::NumericLiteral{ value: "123.456".into()}},
            Test { source: "1", expected: Token::NumericLiteral{ value: "1".into()}},
            Test { source: "0", expected: Token::NumericLiteral{ value: "0".into()}},
            Test { source: "1234567890", expected: Token::NumericLiteral{ value: "1234567890".into()}},
            Test { source: "0.123456789", expected: Token::NumericLiteral{ value: "0.123456789".into()}},
        ];
        for test in tests {
            let tokens = tokenize(test.source).unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0], test.expected);
        }
    }

    #[test]
    fn ignore_whitespace() {
        struct Test {
            source: &'static str,
            expected: Token,
        }
        let tests = [
            Test { source: "   123.456", expected: Token::NumericLiteral{ value: "123.456".into()}},
            Test { source: "1 ", expected: Token::NumericLiteral{ value: "1".into()}},
            Test { source: "\n0\n", expected: Token::NumericLiteral{ value: "0".into()}},
            Test { source: "\n  1234567890\t", expected: Token::NumericLiteral{ value: "1234567890".into()}},
            Test { source: " 0.123456789 ", expected: Token::NumericLiteral{ value: "0.123456789".into()}},
        ];
        for test in tests {
            let tokens = tokenize(test.source).unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0], test.expected);
        }
    }

    #[test]
    fn multiple_tokens() {
        struct Test {
            source: &'static str,
            expected: Vec<Token>,
        }
        let tests = [
            Test { source: "   123.456 2", expected: vec![
                Token::NumericLiteral{ value: "123.456".into()},
                Token::NumericLiteral{ value: "2".into()},
            ]},
            Test { source: "1 2", expected: vec![
                Token::NumericLiteral{ value: "1".into()},
                Token::NumericLiteral{ value: "2".into()},
            ]},
            Test { source: "\n0\n123.65", expected: vec![
                Token::NumericLiteral{ value: "0".into()},
                Token::NumericLiteral{ value: "123.65".into()},
            ]},
            Test { source: "\n  123456 7890\t", expected: vec![
                Token::NumericLiteral{ value: "123456".into()},
                Token::NumericLiteral{ value: "7890".into()},
            ]},
            Test { source: " 0.1234 56789 123\n 0 432.10 89", expected: vec![
                Token::NumericLiteral{ value: "0.1234".into()},
                Token::NumericLiteral{ value: "56789".into()},
                Token::NumericLiteral{ value: "123".into()},
                Token::NumericLiteral{ value: "0".into()},
                Token::NumericLiteral{ value: "432.10".into()},
                Token::NumericLiteral{ value: "89".into()},
            ]},
        ];
        for test in tests {
            let tokens = tokenize(test.source).unwrap();
            assert_eq!(tokens, test.expected);

        }
    }
    
    #[test]
    fn tokenize_operators() {
        struct Test {
            source: &'static str,
            expected: Vec<Token>,
        }
        let tests = [
            Test { source: "+", expected: vec![
                Token::Plus,
            ]},
            Test { source: "-", expected: vec![
                Token::Minus,
            ]},
            Test { source: "*", expected: vec![
                Token::Astrix,
            ]},
            Test { source: "/", expected: vec![
                Token::Slash,
            ]},
        ];
        for test in tests {
            let tokens = tokenize(test.source).unwrap();
            assert_eq!(tokens, test.expected);

        }
    }
    
}
