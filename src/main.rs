mod rule_parser;
mod rules;
mod tokenizer;

use ansi_term::Colour;
use rules::prod;
use tokenizer::{tokenizer, TokenType};
use rule_parser::parse_grammar;

use std::fs::read_to_string;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1..].len() != 2 {
        println!("Usage: {} <grammar file> <file to test>", args[0]);
        exit(1);
    }

    let grammar = read_to_string(args[1].clone()).expect("Grammar file not found");
    let entry = read_to_string(args[2].clone()).expect("File not found");

    let grammar = parse_grammar(grammar);
    let tokens = tokenizer(&entry);

    let valid = prod(tokens);

    println!(
        "\n---------------\nDoes the file complies with the grammar? {}\n---------------",
        if valid {
            Colour::Green.underline().paint("Yes!!")
        } else {
            Colour::Red.underline().paint("Nope :(")
        }
    );
}
