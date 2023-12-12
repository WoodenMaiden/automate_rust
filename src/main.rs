mod rules;
mod tokenizer;

use rules::prod;
use tokenizer::{tokenizer, TokenType};

static ENTRY: &str = r#"

contact A B 20 50
rate 5 20 35
rate 10 35 50
delay 1 20 50 


contact B C 100 140
rate 10 100 140
delay 1 100 130
delay 2 130 140

"#;

fn main() {
    let tokens = tokenizer(ENTRY);


    println!(
        "\nDoes the variable ENTRY respects the grammar? {}",
        prod(tokens)
    );
}
