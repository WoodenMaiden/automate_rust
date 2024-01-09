use crate::TokenType;
use ansi_term::Colour;

const ERR: Colour = Colour::Red;
const SUC: Colour = Colour::Green;

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
    s(&tokens, 0).is_some()
}

/// S -> C
fn s(tokens: &[TokenType], cursor: usize) -> Option<usize> {
    c(tokens, cursor)
}

/// Tests if the tokens at position cursor match the rule C
/// C -> contact <id> <id> <num> <num> \n D |  contact <id> <id> <num> <num> \n R | None
///
/// # Arguments
/// - tokens: vector of tokens {TokenType}
/// - cursor: current position in the vector
///
/// # Returns
/// - Some(cursor) if the tokens match the rule C, None otherwise, None is equivalent of a -1 in C or other languages
fn c(tokens: &[TokenType], cursor: usize) -> Option<usize> {
    let mut cursor = cursor;

    println!(
        "cursor position: {}",
        Colour::Blue.bold().paint(cursor.to_string())
    );

    // The epsillon case
    if cursor == tokens.len() {
        println!("C: {}", SUC.bold().paint("Epsillon case !"));

        return Some(cursor);
    };

    let to_match = tokens.get(cursor..cursor + 6)?;

    print!("C: {:?}", to_match);

    match to_match {
        [TokenType::Keyword("contact"), TokenType::String, TokenType::String, TokenType::Int, TokenType::Int, TokenType::Linebreak] =>
        {
            cursor += 6;

            println!(" -> {}", SUC.bold().paint("Matched !"));

            // returning anchor
            match d(tokens, cursor) {
                Some(c) => Some(c), // D matches so we return the cursor of D
                None => r(tokens, cursor),
            }
        }
        _ => {
            println!(" -> {}", ERR.bold().paint("Not Matched !"));
            None
        }
    }
}

/// Tests if the tokens at position cursor match the rule R
/// R -> rate <num> <num> <num> \n R | rate <num> <num> <num> \n D | rate <num> <num> <num> \n C
///
/// # Arguments
/// - tokens: vector of tokens {TokenType}
/// - cursor: current position in the vector
///
/// # Returns
/// - Some(cursor) if the tokens match the rule R, None otherwise
fn r(tokens: &[TokenType], cursor: usize) -> Option<usize> {
    let mut cursor = cursor;

    let to_match = tokens.get(cursor..cursor + 5)?;

    print!("R: {:?}", to_match);

    match to_match {
        [TokenType::Keyword("rate"), TokenType::Int, TokenType::Int, TokenType::Int, TokenType::Linebreak] =>
        {
            cursor += 5;

            println!(" -> {}", SUC.bold().paint("Matched !"));

            // returning anchor
            match r(tokens, cursor) {
                Some(c) => Some(c), // if R matches we return the cursor of R
                None => match d(tokens, cursor) {
                    // else we try for d
                    Some(anchor_d) => Some(anchor_d),
                    None => c(tokens, cursor), // else we try with C
                },
            }
        }
        _ => {
            println!(" -> {}", ERR.bold().paint("Not Matched !"));
            None
        }
    }
}

/// Tests if the tokens at position cursor match the rule D
/// D -> delay <num> <num> <num> \n R | delay <num> <num> <num> \n D | delay <num> <num> <num> \n C
///
/// # Arguments
/// - tokens: vector of tokens {TokenType}
///
/// # Returns
/// - Some(cursor) if the tokens match the rule D, None otherwise
fn d(tokens: &[TokenType], cursor: usize) -> Option<usize> {
    let mut cursor = cursor;

    let to_match = tokens.get(cursor..cursor + 5)?;

    print!("D: {:?}", to_match);

    match to_match {
        [TokenType::Keyword("delay"), TokenType::Int, TokenType::Int, TokenType::Int, TokenType::Linebreak] =>
        {
            cursor += 5;

            println!(" -> {}", SUC.bold().paint("Matched !"));

            // returning anchor
            match r(tokens, cursor) {
                Some(c) => Some(c), // if R matches we return the cursor of R
                None => match d(tokens, cursor) {
                    // else we try for d
                    Some(anchor_d) => Some(anchor_d),
                    None => c(tokens, cursor), // else we try with C
                },
            }
        }
        _ => {
            println!(" -> {}", ERR.bold().paint("Not Matched !"));
            None
        }
    }
}
