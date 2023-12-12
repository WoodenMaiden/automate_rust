
/// Represents a token type
/// Keyword takes a string as argument to represent its name
/// Useful for pattern matching
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    String,
    Int,
    Keyword(String),
    Linebreak,
}

/// Transforms a string into a vector of tokens
/// 
/// # Arguments
/// - entry: string to parse
/// 
/// # Returns
/// - vector of tokens {TokenType}
pub fn tokenizer(entry: &str) -> Vec<TokenType> {

    let mut tokens = Vec::<TokenType>::new();

    let nb_lines = entry.lines().count();

    for (nb, l) in entry.lines().enumerate() {
        for w in l.split(' ') {
            if w.is_empty() {
                continue;
            }

            match w {
                "contact" => tokens.push(TokenType::Keyword("contact".to_string())),
                "rate" => tokens.push(TokenType::Keyword("rate".to_string())),
                "delay" => tokens.push(TokenType::Keyword("delay".to_string())),
                _ => match w.parse::<i32>() {
                    Ok(_) => tokens.push(TokenType::Int),
                    Err(_) => tokens.push(TokenType::String),
                },
            }
        }

        if nb < nb_lines - 1 {
            tokens.push(TokenType::Linebreak);
        }
    }

    tokens
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"contact A B 20 50
rate 5 20 35"#;

    #[test]
    fn test_tokenizer() {
        let tokens = tokenizer(EXAMPLE1);

        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens, vec![
            TokenType::Keyword("contact".to_string()),
            TokenType::String,
            TokenType::String,
            TokenType::Int,
            TokenType::Int,
            TokenType::Linebreak,
            TokenType::Keyword("rate".to_string()),
            TokenType::Int,
            TokenType::Int,
            TokenType::Int])
    }
}