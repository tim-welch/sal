use std::error::Error;

pub struct Token {}

pub fn tokenize(_source: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_returns_no_tokens() {
        let tokens = tokenize("").unwrap();
        assert_eq!(tokens.len(), 0);
    }
}
