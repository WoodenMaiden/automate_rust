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

}

fn S(){
    
}
