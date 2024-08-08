use super::base::lexer::{self, TokenDefinition};

fn get_token_definitions() -> Vec<lexer::TokenDefinition> {
    let definitions = vec![TokenDefinition::new(
        "variable",
        r"(:|\$)?[a-zA-Z_][a-zA-Z_0-9]*",
    )];

    return definitions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_variable() {
        let token_definitions = get_token_definitions();
        
        let mut lexer = lexer::Lexer::new("VIceSscaled", 0, token_definitions.iter());

        let token = lexer.get_next_token().expect("Should find token.");
        
        assert!(std::ptr::eq(token.definition, &token_definitions[0]));
    }

    #[test]
    fn test_not_a_variable() {
        let token_definitions = get_token_definitions();
        
        let mut lexer = lexer::Lexer::new("9*VIceSscaled", 0, token_definitions.iter());

        let option = lexer.get_next_token();

        assert!(option.is_none())
    }
}
