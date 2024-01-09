use ansi_term::Colour;

/// Represents a token type
/// Keyword takes a string as argument to represent its name
/// Useful for pattern matching
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType<'a> {
    /// A variable string, identified by <id> in the grammar
    String,
    /// An integer, identified by <num> in the grammar
    Int,
    /// A keyword, identified by its name in the grammar
    Keyword(&'a str),
    /// A linebreak, identified by \n in the grammar
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

    for l in entry.lines() {
        if l.is_empty() || l.starts_with('\n') {
            continue;
        }

        for w in l.split(' ') {
            if w.is_empty() {
                continue;
            }

            match w {
                "contact" => tokens.push(TokenType::Keyword("contact")),
                "rate" => tokens.push(TokenType::Keyword("rate")),
                "delay" => tokens.push(TokenType::Keyword("delay")),
                _ => match w.parse::<i32>() {
                    Ok(_) => tokens.push(TokenType::Int),
                    Err(_) => tokens.push(TokenType::String),
                },
            }
        }

        tokens.push(TokenType::Linebreak);
    }

    println!(
        "Got {} tokens",
        Colour::Blue.bold().paint(tokens.len().to_string())
    );
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"contact A B 20 50
rate 5 20 35

delay A 3
"#;

    #[test]
    fn test_tokenizer() {
        let tokens = tokenizer(EXAMPLE1);

        assert_eq!(
            tokens,
            vec![
                TokenType::Keyword("contact"),
                TokenType::String,
                TokenType::String,
                TokenType::Int,
                TokenType::Int,
                TokenType::Linebreak,
                TokenType::Keyword("rate"),
                TokenType::Int,
                TokenType::Int,
                TokenType::Int,
                TokenType::Linebreak,
                TokenType::Keyword("delay"),
                TokenType::String,
                TokenType::Int,
                TokenType::Linebreak,
            ]
        )
    }
}
