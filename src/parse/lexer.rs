use super::base::lexer::{self, TokenDefinition};

pub struct TokenDefinitions {
    variable: TokenDefinition,
}

impl TokenDefinitions {
    pub fn new() -> TokenDefinitions {
        return TokenDefinitions {
            variable: TokenDefinition::new("variable", r"(:|\$)?[a-zA-Z_][a-zA-Z_0-9]*"),
        };
    }

    pub fn all(&self) -> Vec<&TokenDefinition> {
        vec![&self.variable]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_variable() {
        let token_definitions = TokenDefinitions::new();

        let mut lexer = lexer::Lexer::new("VIceSscaled", 0, token_definitions.all().into_iter());

        let token = lexer.get_next_token().expect("Should find token.");

        assert!(std::ptr::eq(token.definition, &token_definitions.variable));
    }

    #[test]
    fn test_not_a_variable() {
        let token_definitions = TokenDefinitions::new();

        let mut lexer = lexer::Lexer::new("9*VIceSscaled", 0, token_definitions.all().into_iter());

        let option = lexer.get_next_token();

        assert!(option.is_none())
    }
}
