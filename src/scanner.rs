use rpds::Vector;
use std::error::Error;

type Source = Vec<char>;
type Tokens = Vector<Token>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    EOF,

    // Literals
    NumericLiteral { value: String },

    // Punctuation
    OpenParen,
    CloseParen,

    // Operators
    Plus,
    Minus,
    Astrix,
    Slash,

    // Identifiers
    Identifier { value: String },
}

const PUNCTUATION: &[char] = &['(', ')'];

// TODO: Make Lexer an iterator and remove mutable used variable
// TODO: Use map(?) to build vector of tokens from Lexer?
// TODO: Don't build vector of tokens, just pass Lexer to parse?

pub struct Lexer<'a> {
    source: &'a [char],
}

fn is_end(lex: &Lexer, used: usize) -> bool {
    lex.source.len() <= used
}

fn is_whitespace(lex: &Lexer, current: usize) -> bool {
    lex.source[current].is_whitespace()
}

fn is_punctuation(lex: &Lexer, current: usize) -> bool {
    PUNCTUATION.contains(&lex.source[current])
}

fn number<'a>(lex: &'a Lexer) -> (Lexer<'a>, Option<Token>) {
    let mut used = 0;
    while !is_end(lex, used) && lex.source[used].is_ascii_digit() {
        used += 1;
    }
    if !is_end(lex, used) && lex.source[used] == '.' {
        used += 1;
    }
    while !is_end(lex, used) && lex.source[used].is_ascii_digit() {
        used += 1;
    }

    (
        Lexer {
            source: &(lex.source[used..]),
        },
        Some(Token::NumericLiteral {
            value: lex.source[..used].iter().collect(),
        }),
    )
}

fn identifier<'a>(lex: &'a Lexer) -> (Lexer<'a>, Option<Token>) {
    let mut used = 0;
    while !is_end(lex, used) && !is_whitespace(lex, used) && !is_punctuation(lex, used) {
        used += 1;
    }

    (
        Lexer {
            source: &(lex.source[used..]),
        },
        Some(Token::Identifier {
            value: lex.source[..used].iter().collect(),
        }),
    )
}

fn eat_whitespace<'a>(lex: &'a Lexer) -> Option<Lexer<'a>> {
    let mut used: usize = 0;
    while !is_end(lex, used) && is_whitespace(lex, used) {
        used += 1;
    }
    if used > 0 {
        Some(Lexer {
            source: &(lex.source[used..]),
        })
    } else {
        None
    }
}

fn next_token<'a>(lex: &'a Lexer) -> Result<(Lexer<'a>, Option<Token>), Box<dyn Error>> {
    match lex.source[0] {
        '0'..='9' => Ok(number(lex)),
        '+' => Ok((
            Lexer {
                source: &(lex.source[1..]),
            },
            Some(Token::Plus),
        )),
        '-' => Ok((
            Lexer {
                source: &(lex.source[1..]),
            },
            Some(Token::Minus),
        )),
        '*' => Ok((
            Lexer {
                source: &(lex.source[1..]),
            },
            Some(Token::Astrix),
        )),
        '/' => Ok((
            Lexer {
                source: &(lex.source[1..]),
            },
            Some(Token::Slash),
        )),
        '(' => Ok((
            Lexer {
                source: &(lex.source[1..]),
            },
            Some(Token::OpenParen),
        )),
        ')' => Ok((
            Lexer {
                source: &(lex.source[1..]),
            },
            Some(Token::CloseParen),
        )),
        _ => {
            if let Some(lex) = eat_whitespace(lex) {
                Ok((lex, None))
            } else {
                Ok(identifier(lex))
            }
        }
    }
}

fn do_tokenize(lex: &Lexer, tokens: Tokens) -> Result<Tokens, Box<dyn Error>> {
    if is_end(lex, 0) {
        Ok(tokens)
    } else {
        let next = next_token(lex)?;
        let new_tokens = if let Some(token) = next.1 {
            tokens.push_back(token)
        } else {
            tokens
        };
        do_tokenize(&next.0, new_tokens)
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let source: Source = source.chars().collect();
    let tokens = Tokens::new();
    let lex = Lexer {
        source: &source[..],
    };

    let tokens = do_tokenize(&lex, tokens)?;
    Ok(tokens.iter().cloned().collect())
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
            Test {
                source: "123.456",
                expected: Token::NumericLiteral {
                    value: "123.456".into(),
                },
            },
            Test {
                source: "1",
                expected: Token::NumericLiteral { value: "1".into() },
            },
            Test {
                source: "0",
                expected: Token::NumericLiteral { value: "0".into() },
            },
            Test {
                source: "1234567890",
                expected: Token::NumericLiteral {
                    value: "1234567890".into(),
                },
            },
            Test {
                source: "0.123456789",
                expected: Token::NumericLiteral {
                    value: "0.123456789".into(),
                },
            },
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
            Test {
                source: "   123.456",
                expected: Token::NumericLiteral {
                    value: "123.456".into(),
                },
            },
            Test {
                source: "1 ",
                expected: Token::NumericLiteral { value: "1".into() },
            },
            Test {
                source: "\n0\n",
                expected: Token::NumericLiteral { value: "0".into() },
            },
            Test {
                source: "\n  1234567890\t",
                expected: Token::NumericLiteral {
                    value: "1234567890".into(),
                },
            },
            Test {
                source: " 0.123456789 ",
                expected: Token::NumericLiteral {
                    value: "0.123456789".into(),
                },
            },
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
            Test {
                source: "   123.456 2",
                expected: vec![
                    Token::NumericLiteral {
                        value: "123.456".into(),
                    },
                    Token::NumericLiteral { value: "2".into() },
                ],
            },
            Test {
                source: "1 2",
                expected: vec![
                    Token::NumericLiteral { value: "1".into() },
                    Token::NumericLiteral { value: "2".into() },
                ],
            },
            Test {
                source: "\n0\n123.65",
                expected: vec![
                    Token::NumericLiteral { value: "0".into() },
                    Token::NumericLiteral {
                        value: "123.65".into(),
                    },
                ],
            },
            Test {
                source: "\n  123456 7890\t",
                expected: vec![
                    Token::NumericLiteral {
                        value: "123456".into(),
                    },
                    Token::NumericLiteral {
                        value: "7890".into(),
                    },
                ],
            },
            Test {
                source: " 0.1234 56789 123\n 0 432.10 89",
                expected: vec![
                    Token::NumericLiteral {
                        value: "0.1234".into(),
                    },
                    Token::NumericLiteral {
                        value: "56789".into(),
                    },
                    Token::NumericLiteral {
                        value: "123".into(),
                    },
                    Token::NumericLiteral { value: "0".into() },
                    Token::NumericLiteral {
                        value: "432.10".into(),
                    },
                    Token::NumericLiteral { value: "89".into() },
                ],
            },
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
            Test {
                source: "+",
                expected: vec![Token::Plus],
            },
            Test {
                source: "-",
                expected: vec![Token::Minus],
            },
            Test {
                source: "*",
                expected: vec![Token::Astrix],
            },
            Test {
                source: "/",
                expected: vec![Token::Slash],
            },
        ];
        for test in tests {
            let tokens = tokenize(test.source).unwrap();
            assert_eq!(tokens, test.expected);
        }
    }

    #[test]
    fn tokenize_punctuation() {
        struct Test {
            source: &'static str,
            expected: Vec<Token>,
        }
        let tests = [
            Test {
                source: "(",
                expected: vec![Token::OpenParen],
            },
            Test {
                source: ")",
                expected: vec![Token::CloseParen],
            },
        ];
        for test in tests {
            let tokens = tokenize(test.source).unwrap();
            assert_eq!(tokens, test.expected);
        }
    }

    #[test]
    fn tokenize_identifiers() {
        struct Test {
            source: &'static str,
            expected: Vec<Token>,
        }
        let tests = [
            Test {
                source: "x",
                expected: vec![Token::Identifier { value: "x".into() }],
            },
            Test {
                source: "abc123",
                expected: vec![Token::Identifier {
                    value: "abc123".into(),
                }],
            },
            Test {
                source: "abc123)",
                expected: vec![
                    Token::Identifier {
                        value: "abc123".into(),
                    },
                    Token::CloseParen,
                ],
            },
            Test {
                source: "abc123 ",
                expected: vec![Token::Identifier {
                    value: "abc123".into(),
                }],
            },
            Test {
                source: "(a_b+c-1'2!3)",
                expected: vec![
                    Token::OpenParen,
                    Token::Identifier {
                        value: "a_b+c-1'2!3".into(),
                    },
                    Token::CloseParen,
                ],
            },
        ];
        for test in tests {
            let tokens = tokenize(test.source).unwrap();
            assert_eq!(tokens, test.expected);
        }
    }
}
