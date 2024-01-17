pub mod rule_parser;
pub mod rules;
pub mod tokenizer;

use rule_parser::parse_grammar;
use rules::{apply_grammar, ERR, INF, SUC};
use tokenizer::{tokenizer, TokenType};

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

    println!(
        "{}",
        INF.italic()
            .paint(format!("Grammar is: 👇\n{:?}\n", grammar))
    );

    println!(
        "{}",
        INF.italic()
            .paint(format!("File to test is: 👇\n{:?}\n", tokens))
    );

    let valid = apply_grammar(&tokens[..], grammar);

    println!(
        "\n---------------\nDoes the file complies with the grammar? {}\n---------------",
        if valid {
            SUC.bold().paint("Yes!!")
        } else {
            ERR.bold().paint("Nope!")
        }
    );
}
