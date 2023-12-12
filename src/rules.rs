use crate::TokenType;

// S -> C
// C -> contact <id> <id> <num> <num> \n D |  contact <id> <id> <num> <num> \n R | None
// R -> rate <num> <num> <num> \n R | rate <num> <num> <num> \n D | rate <num> <num> <num> \n C
// D -> delay <num> <num> <num> \n R | delay <num> <num> <num> \n D | delay <num> <num> <num> \n C

/// Runs the grammar checker
///
/// # Arguments
/// - tokens: vector of tokens {TokenType}
///
/// # Returns
/// - true if the tokens respect the gramar, false otherwise
pub fn prod(tokens: Vec<TokenType>) -> bool {
    s(&tokens)
}

/// Represents the S rule
/// S -> C
///
/// # Arguments
/// - remainng_tokens: vector of tokens {TokenType}
///
/// # Returns
/// - true if the tokens respect the rule, false otherwise
fn s(remaining_tokens: &[TokenType]) -> bool {
    c(remaining_tokens)
}

/// Represents the C rule
/// C -> contact <id> <id> <num> <num> \n D |  contact <id> <id> <num> <num> \n R | None
///
/// # Arguments
/// - remaining_tokens: vector of tokens {TokenType}
///
/// # Returns
/// - true if the tokens respect the rule, false otherwise
fn c(remaining_tokens: &[TokenType]) -> bool {
    println!("Called C w/ remaining_tokens: {:?}", remaining_tokens);
    // since it can be NONE it will respect the rule
    if remaining_tokens.is_empty() {
        return true;
    };

    // if we have either too few tokens to respect the rule AND if we have more than 0 tokens we won't respect the rule
    if remaining_tokens.len() < 6 {
        return false;
    };

    let tokens_concerned = &remaining_tokens[..6];

    println!("tokens_concerned: {:?}", tokens_concerned);

    match tokens_concerned {
        [TokenType::Keyword("contact"), TokenType::String, TokenType::String, TokenType::Int, TokenType::Int, TokenType::Linebreak] => {
            d(&remaining_tokens[6..]) || r(&remaining_tokens[6..])
        }
        _ => false,
    }
}

/// Represents the R rule
/// R -> rate <num> <num> <num> \n R | rate <num> <num> <num> \n D | rate <num> <num> <num> \n C
///
/// # Arguments
/// - remaining_tokens: vector of tokens {TokenType}
///
/// # Returns
/// - true if the tokens respect the rule, false otherwise
fn r(remaining_tokens: &[TokenType]) -> bool {
    println!("Called R w/ remaining_tokens: {:?}", remaining_tokens);

    if remaining_tokens.len() < 5 {
        return false;
    };

    let tokens_concerned = &remaining_tokens[..5];

    println!("tokens_concerned: {:?}", tokens_concerned);

    match tokens_concerned {
        [TokenType::Keyword("rate"), TokenType::Int, TokenType::Int, TokenType::Int, TokenType::Linebreak] => {
            r(&remaining_tokens[5..]) || d(&remaining_tokens[5..]) || c(&remaining_tokens[5..])
        }
        _ => false,
    }
}

/// Represents the D rule
/// D -> delay <num> <num> <num> \n R | delay <num> <num> <num> \n D | delay <num> <num> <num> \n C
///
/// # Arguments
/// - remaining_tokens: vector of tokens {TokenType}
///
/// # Returns
/// - true if the tokens respect the rule, false otherwise
fn d(remaining_tokens: &[TokenType]) -> bool {
    println!("Called D w/ remaining_tokens: {:?}", remaining_tokens);

    if remaining_tokens.len() < 5 {
        return false;
    };

    let tokens_concerned = &remaining_tokens[..5];

    println!("tokens_concerned: {:?}", tokens_concerned);

    match tokens_concerned {
        [TokenType::Keyword("delay"), TokenType::Int, TokenType::Int, TokenType::Int, TokenType::Linebreak] => {
            r(&remaining_tokens[5..]) || d(&remaining_tokens[5..]) || c(&remaining_tokens[5..])
        }
        _ => false,
    }
}
