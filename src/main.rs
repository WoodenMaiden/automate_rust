mod rules;
mod tokenizer;

use ansi_term::Colour;
use rules::prod;
use tokenizer::{tokenizer, TokenType};

use std::{env, process::exit};
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        exit(1);
    }

    let entry = read_to_string(args[1].clone()).expect("File not found");

    let tokens = tokenizer(&entry);

    let valid = prod(tokens);

    println!(
        "\n---------------\nDoes the variable ENTRY respects the grammar? {}",
        if valid {
            Colour::Green.underline().paint("Yes")
        } else {
            Colour::Red.underline().paint("No")
        }
    );
}
