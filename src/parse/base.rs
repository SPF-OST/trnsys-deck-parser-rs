#[cfg(feature = "python-bindings")]
use pyo3::prelude::*;

pub struct TokenDefinition {
    description: String,
    regex: regex::Regex,
    #[allow(dead_code)]
    priority: i32,
}

#[derive(Clone)]
pub struct Token<'t, 'i> {
    definition: &'t TokenDefinition,
    value: &'i str,
}

impl TokenDefinition {
    pub fn new_custom(
        description: &str,
        regex_pattern: &str,
        case_sensitive: bool,
        priority: i32,
    ) -> TokenDefinition {
        let mut builder = regex::RegexBuilder::new(regex_pattern);
        let case_insensitive = !case_sensitive;
        let regex = builder
            .case_insensitive(case_insensitive)
            .build()
            .expect("The regex pattern is valid.");

        TokenDefinition {
            description: String::from(description),
            regex,
            priority,
        }
    }

    pub fn new(description: &str, regex_pattern: &str) -> TokenDefinition {
        let case_sensitive = false;
        let priority = -1;

        Self::new_custom(description, regex_pattern, case_sensitive, priority)
    }
}

pub struct Lexer<'t, 'i> {
    input: &'i str,
    current_pos: usize,
    token_definitions: Vec<&'t TokenDefinition>,
}

#[cfg_attr(feature = "python-bindings", pyclass(get_all))]
#[derive(PartialEq, Debug)]
pub struct Error {
    pub message: String,
    input_pos: usize,
}

impl<'t, 'i> Lexer<'t, 'i> {
    pub fn new(
        input: &'i str,
        current_pos: usize,
        token_definitions: Vec<&'t TokenDefinition>,
    ) -> Lexer<'t, 'i> {
        Lexer {
            input,
            current_pos,
            token_definitions,
        }
    }

    pub fn get_next_token<'l>(&'l mut self) -> Option<Token<'t, 'i>> {
        for token_definition in &self.token_definitions {
            match (token_definition.regex).find(&self.input) {
                Some(m) => {
                    self.current_pos = m.end();

                    let value = m.as_str();
                    let token = Token {
                        value,
                        definition: token_definition,
                    };
                    return Some(token);
                }
                None => (),
            }
        }

        return None;
    }
}

pub struct ParserBase<'t, 'i> {
    lexer: Lexer<'t, 'i>,
    current_token: Option<Token<'t, 'i>>,
}

impl<'t, 'i> ParserBase<'t, 'i> {
    pub fn accept(&mut self, token_definition: &'t TokenDefinition) -> Option<Token<'t, 'i>> {
        if self.current_token.is_none() {
            let option = self.lexer.get_next_token();

            if option.is_none() {
                return None;
            }

            self.current_token = option;
        }

        let token = Self::get_current_token(&self.current_token);

        if !std::ptr::eq(token.definition, token_definition) {
            return None;
        }

        return Some(token);
    }

    pub fn expect(
        &mut self,
        token_definition: &'t TokenDefinition,
    ) -> Result<Token<'t, 'i>, Error> {
        if self.current_token.is_none() {
            let option = self.lexer.get_next_token();

            if option.is_none() {
                let error_message = format!(
                    "Expected {descr} but there wasn't any next token.",
                    descr = token_definition.description
                );

                let input_pos = self.lexer.current_pos;

                let error = Error {
                    message: error_message,
                    input_pos,
                };
                return Err(error);
            }

            self.current_token = option;
        }

        let token = Self::get_current_token(&self.current_token);

        if std::ptr::eq(token.definition, token_definition) {
            return Ok(token);
        }

        let error_message = format!(
            "Expected {expected_descr} bout found {actual_descr}",
            expected_descr = token_definition.description,
            actual_descr = token.definition.description
        );
        let error = Error {
            message: error_message,
            input_pos: self.lexer.current_pos,
        };

        return Err(error);
    }

    fn get_current_token(option: &Option<Token<'t, 'i>>) -> Token<'t, 'i> {
        option.as_ref().expect("Current token to be set.").clone()
    }
}
