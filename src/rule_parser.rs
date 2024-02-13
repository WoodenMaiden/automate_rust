use std::collections::HashMap;

use ansi_term::Colour;

use crate::tokenizer::TokenType;
use serde::{Deserialize, Serialize};

/// These are the components of a rule
#[derive(Debug, PartialEq, Clone)]
pub enum RuleToken<'a> {
    /// This is a string, typically represented between &lt; and &gt; characters
    /// See more on [TokenType](crate::tokenizer::TokenType)
    Token(TokenType<'a>),
    /// This is a non-terminal character, typically represented by a capital letter
    Rule(String),
    // Epsilon
    None,
}

/// This is the grammar structure
/// It contains the initial state and the rules
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Grammar<'a> {
    /// This is where the grammar starts at, by default its the first rule ever inserted, meaning on top of the file
    pub init_state: String,
    /// The rules of the grammar, the key is the non-terminal character, the value is a vector of all the possible rules
    pub rules: HashMap<String, Vec<Vec<RuleToken<'a>>>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct RuleJSONEntry<'a> {
    non_terminal: &'a str,
    content: &'a str,
}

impl Grammar<'_> {
    pub fn new() -> Grammar<'static> {
        Grammar {
            init_state: String::new(),
            rules: HashMap::new(),
        }
    }
}

pub fn parse_grammar(input: Vec<RuleJSONEntry<'_>>) -> Grammar<'_> {
    let non_terms: Vec<&str> = input.clone().into_iter().map(|e| e.non_terminal).collect();

    let rules = input.iter().map(|entry| {
        let content = &entry.content;
        let possibilities: Vec<Vec<RuleToken>> = content
            .split(" | ")
            .map(|c| {
                c.split(' ')
                    .filter(|token| !token.is_empty())
                    .map(|token| {
                        if non_terms.contains(&token) {
                            RuleToken::Rule(String::from(token))
                        } else {
                            match token {
                                "None" => RuleToken::None,
                                "<str>" | "<id>" => RuleToken::Token(TokenType::String),
                                "<num>" => RuleToken::Token(TokenType::Int),
                                "<br>" => RuleToken::Token(TokenType::Linebreak),
                                _ => RuleToken::Token(TokenType::Keyword(token)),
                            }
                        }
                    })
                    .collect::<Vec<RuleToken>>()
            })
            .collect();

        (String::from(entry.non_terminal), possibilities)
    });

    Grammar {
        init_state: input
            .first()
            .unwrap_or_else(|| {
                panic!(
                    "{}",
                    Colour::Red.bold().paint("There shall be at least ont rule")
                )
            })
            .non_terminal.to_string(),
        rules: HashMap::from_iter(rules),
    }
}

// test
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        let rule: Vec<RuleJSONEntry> = serde_json::from_str(
            r#"[
                {
                  "non_terminal": "S",
                  "content": "C S | C"
                },
                {
                  "non_terminal": "C",
                  "content": "contact <id> <id> <num> <num> <br> E"
                },
                {
                  "non_terminal": "E",
                  "content": "R E | D E | None"
                },
                {
                  "non_terminal": "R",
                  "content": "rate <num> <num> <num> <br>"
                },
                {
                  "non_terminal": "D",
                  "content": "delay <num> <num> <num> <br>"
                }
              ]
              "#,
        )
        .unwrap();

        let grammar = parse_grammar(rule);

        assert_eq!(grammar.init_state, "S");

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
                RuleToken::Token(TokenType::Keyword("contact")),
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

        assert_eq!(grammar.rules.get("E").unwrap()[2], vec![RuleToken::None]);

        assert_eq!(grammar.rules.get("R").unwrap().len(), 1);
        assert_eq!(
            grammar.rules.get("R").unwrap()[0],
            vec![
                RuleToken::Token(TokenType::Keyword("rate")),
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
                RuleToken::Token(TokenType::Keyword("delay")),
                RuleToken::Token(TokenType::Int),
                RuleToken::Token(TokenType::Int),
                RuleToken::Token(TokenType::Int),
                RuleToken::Token(TokenType::Linebreak)
            ]
        );
    }
}
