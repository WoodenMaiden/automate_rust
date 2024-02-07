use crate::{
    rule_parser::{Grammar, RuleToken},
    TokenType,
};
use ansi_term::Colour;

pub const ERR: Colour = Colour::Red;
pub const SUC: Colour = Colour::Green;
pub const INF: Colour = Colour::Blue;

/// It runs [apply_rule](crate::rules::apply_rule) on all the rules of the initial state, it acts as an entrypoint
///
/// # Arguments
///
/// * `tokens` - The tokens to match
/// * `grammar` - The grammar to match against
///
/// # Returns
///
/// * `bool` - Whether the tokens match the grammar or not
pub fn apply_grammar(tokens: &[TokenType], grammar: Grammar) -> bool {
    let results: Vec<Option<usize>> = grammar
        .rules
        .get(&grammar.init_state)
        .iter()
        .map(|rules| {
            rules
                .iter()
                .map(|r| apply_rule(tokens, r.clone(), grammar.clone(), 0))
                .find(|r| r.is_some())?
        })
        .collect();

    println!("results: {:?}", results);

    results.iter().any(|r| r.is_some())
}

/// This function tries to match a rule against a vector of tokens
///
/// # Arguments
///
/// * `tokens` - The tokens to match
/// * `rule` - The rule to match against
/// * `grammar` - The underlying grammar
/// * `cursor` - The current cursor position
///
/// # Returns
///
/// * `Option<usize>` - The cursor position if the rule matches, None otherwise
///
fn apply_rule(
    tokens: &[TokenType],
    rule: Vec<RuleToken>,
    grammar: Grammar,
    cursor: usize,
) -> Option<usize> {
    let mut cursor = cursor;

    let to_match = tokens.get(cursor..)?;

    for (token_index, token) in rule.iter().enumerate() {
        println!("I am trying token: {:?}", token);

        match token {
            RuleToken::Rule(rule_term) => {
                let rule = grammar.rules.get(&rule_term.clone())?;

                println!("I am trying all possibilities of rule: {}\n", rule_term);

                return rule
                    .iter()
                    .map(|r| apply_rule(tokens, r.clone(), grammar.clone(), cursor))
                    .find(|r| r.is_some())?;
            }
            RuleToken::None => {
                // The epsilon case, if epsilon is the last token we return the cursor
                if cursor == tokens.len() {
                    println!("{}", SUC.bold().paint("Epsilon case at the end!"));

                    return Some(cursor);
                };

                cursor += 1;
            }
            // Here if we have different tokens we return None
            RuleToken::Token(tok) => {
                if to_match.get(token_index) != Some(tok) {
                    return None;
                } else if token_index == rule.len() - 1 {
                    println!("{}", SUC.bold().paint(format!("Rule {:?} matches!", rule)));

                    return Some(cursor);
                } else {
                    cursor += 1;
                }
            }
        };
    }

    None
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, iter};

    use crate::{
        rule_parser::{Grammar, RuleToken},
        rules::apply_grammar,
        tokenizer::TokenType,
    };

    #[test]
    fn test_epsilon_works() {
        let mut epsilon_grammar = Grammar::new();
        epsilon_grammar.init_state = "S".to_string();
        epsilon_grammar.rules =
            HashMap::from_iter(iter::once(("S".to_string(), vec![vec![RuleToken::None]])));

        let tokens = vec![];

        assert!(apply_grammar(&tokens, epsilon_grammar));
    }

    #[test]
    fn test_matching_string() {
        let mut grammar = Grammar::new();
        grammar.init_state = "S".to_string();
        grammar.rules = HashMap::from_iter(iter::once((
            "S".to_string(),
            vec![vec![RuleToken::Token(TokenType::String)]],
        )));

        let tokens = vec![TokenType::String];

        assert!(apply_grammar(&tokens, grammar));
    }

    #[test]
    fn terminal_having_several_possibilities() {
        let mut grammar = Grammar::new();
        grammar.init_state = "S".to_string();
        grammar.rules = HashMap::from_iter(iter::once((
            "S".to_string(),
            vec![
                vec![RuleToken::Token(TokenType::String)],
                vec![RuleToken::Token(TokenType::Int)],
            ],
        )));

        let tokens = vec![TokenType::String];

        assert!(apply_grammar(&tokens, grammar));
    }

    #[test]
    fn test_matching_rule() {
        let mut grammar = Grammar::new();
        grammar.init_state = "S".to_string();
        grammar.rules = HashMap::from_iter(iter::once((
            "S".to_string(),
            vec![vec![
                RuleToken::Token(TokenType::String),
                RuleToken::Rule("A".to_string()),
            ]],
        )));

        grammar.rules.insert(
            "A".to_string(),
            vec![vec![RuleToken::Token(TokenType::Int)]],
        );

        let tokens = vec![TokenType::String, TokenType::Int];

        assert!(apply_grammar(&tokens, grammar));
    }

    #[test]
    fn terminal_leading_to_several_terminals() {
        let mut grammar = Grammar::new();
        grammar.init_state = "S".to_string();

        let mut rules: HashMap<String, Vec<Vec<RuleToken>>> = HashMap::new();

        rules.insert(
            "S".to_string(),
            vec![vec![
                RuleToken::Rule("A".to_string()),
                RuleToken::Rule("B".to_string()),
            ]],
        );
        rules.insert(
            "A".to_string(),
            vec![vec![RuleToken::Token(TokenType::Int)]],
        );
        rules.insert(
            "B".to_string(),
            vec![vec![RuleToken::Token(TokenType::String), RuleToken::None]],
        );

        grammar.rules = rules;

        let tokens = vec![TokenType::Int, TokenType::String];

        assert!(apply_grammar(&tokens, grammar));
    }

    #[test]
    fn terminal_with_possibilities_having_several_terminals() {
        let mut grammar = Grammar::new();
        grammar.init_state = "S".to_string();

        let mut rules: HashMap<String, Vec<Vec<RuleToken>>> = HashMap::new();

        rules.insert(
            "S".to_string(),
            vec![vec![
                RuleToken::Rule("A".to_string()),
                RuleToken::Rule("B".to_string()),
            ]],
        );

        rules.insert(
            "A".to_string(),
            vec![vec![RuleToken::Token(TokenType::Int)]],
        );
        rules.insert(
            "B".to_string(),
            vec![vec![RuleToken::Token(TokenType::String), RuleToken::None]],
        );

        grammar.rules = rules;

        let tokens = vec![TokenType::Int, TokenType::String];

        assert!(apply_grammar(&tokens, grammar));
    }
}
