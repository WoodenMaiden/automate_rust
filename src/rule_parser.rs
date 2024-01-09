use std::collections::HashMap;

use ansi_term::Colour;

use crate::tokenizer::TokenType;

pub type Terminal = String;

/// These are the components of a rule
#[derive(Debug, PartialEq, Clone)]
pub enum RuleToken<'a> {
    /// This is a string, typically represented between &lt; and &gt; characters
    /// See more on [TokenType](crate::tokenizer::TokenType)
    Token(TokenType<'a>),
    /// This is a terminal character, typically represented by a capital letter
    Rule(Terminal), 
    // Epsillon
    None
}

/// This is the grammar structure
/// It contains the initial state and the rules
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Grammar<'a> {
    /// This is where the grammar starts at, by default its the first rule ever inserted, meaning on top of the file
    pub init_state: Terminal,
    /// The rules of the grammar, the key is the terminal character, the value is a vector of all the possible rules
    pub rules: HashMap<Terminal, Vec<Vec<RuleToken<'a>>>>,
}

impl Grammar<'_> {
    pub fn new() -> Grammar<'static> {
        Grammar {
            init_state: String::new(),
            rules: HashMap::new(),
        }
    }
}


pub fn parse_grammar(input: String) -> Grammar<'static> {
    let mut grammar = Grammar::new();

    let terms: Vec<String> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| { // we get whats on the left of the arrow
            l.trim()
                .split(" -> ")
                .next()
                .unwrap_or_else(|| {
                    panic!(
                        "{}",
                        Colour::Red
                            .bold()
                            .paint("format: <Terminal character> -> <rule>")
                    )
                })
                .to_string()
        })
        .collect();

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let term = line.trim().split(" -> ").next().unwrap();
        let rule = line.trim().split(" -> ").nth(1).unwrap();
        let possibilities: Vec<Vec<RuleToken>> = rule
            .split(" | ")
            .map(|p| {
                p.split(' ')
                    .filter(|token| !token.is_empty())
                    .map(|token| {
                        if terms.contains(&String::from(token)) {
                            RuleToken::Rule(String::from(token))
                        } else {
                            match token {
                                "None" => RuleToken::None,
                                "<str>" | "<id>" => RuleToken::Token(TokenType::String),
                                "<num>" => RuleToken::Token(TokenType::Int),
                                "\\n" => RuleToken::Token(TokenType::Linebreak),
                                _ => RuleToken::Rule(String::from(token)),
                            }
                        }
                    })
                    .collect::<Vec<RuleToken>>()
            })
            .collect();

        grammar.rules.insert(String::from(term), possibilities);
    }

    grammar
}

// test
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        let rule = String::from(
        r#"S -> C S | C
        C -> contact <id> <id> <num> <num> \n E
        E -> R E | D E | None
        R -> rate <num> <num> <num> \n
        D -> delay <num> <num> <num> \n"#,
        );

        let grammar = parse_grammar(rule);

        assert_eq!(grammar.rules.len(), 5);

        assert_eq!(grammar.rules.get("S").unwrap().len(), 2);
        assert_eq!(
            grammar.rules.get("S").unwrap()[0],
            vec![
                RuleToken::Rule(String::from("C")),
                RuleToken::Rule(String::from("S"))
            ]
        );
        assert_eq!(
            grammar.rules.get("S").unwrap()[1],
            vec![RuleToken::Rule(String::from("C"))]
        );

        assert_eq!(grammar.rules.get("C").unwrap().len(), 1);
        assert_eq!(
            grammar.rules.get("C").unwrap()[0],
            vec![
                RuleToken::Rule(String::from("contact")),
                RuleToken::Token(TokenType::String),
                RuleToken::Token(TokenType::String),
                RuleToken::Token(TokenType::Int),
                RuleToken::Token(TokenType::Int),
                RuleToken::Token(TokenType::Linebreak),
                RuleToken::Rule(String::from("E"))
            ]
        );

        assert_eq!(grammar.rules.get("E").unwrap().len(), 3);
        assert_eq!(
            grammar.rules.get("E").unwrap()[0],
            vec![
                RuleToken::Rule(String::from("R")),
                RuleToken::Rule(String::from("E"))
            ]
        );

        assert_eq!(
            grammar.rules.get("E").unwrap()[1],
            vec![
                RuleToken::Rule(String::from("D")),
                RuleToken::Rule(String::from("E"))
            ]
        );

        assert_eq!(
            grammar.rules.get("E").unwrap()[2],
            vec![RuleToken::None]
        );

        assert_eq!(grammar.rules.get("R").unwrap().len(), 1);
        assert_eq!(
            grammar.rules.get("R").unwrap()[0],
            vec![
                RuleToken::Rule(String::from("rate")),
                RuleToken::Token(TokenType::Int),
                RuleToken::Token(TokenType::Int),
                RuleToken::Token(TokenType::Int),
                RuleToken::Token(TokenType::Linebreak)
            ]
        );

        assert_eq!(grammar.rules.get("D").unwrap().len(), 1);
        assert_eq!(
            grammar.rules.get("D").unwrap()[0],
            vec![
                RuleToken::Rule(String::from("delay")),
                RuleToken::Token(TokenType::Int),
                RuleToken::Token(TokenType::Int),
                RuleToken::Token(TokenType::Int),
                RuleToken::Token(TokenType::Linebreak)
            ]
        );
    }
}
