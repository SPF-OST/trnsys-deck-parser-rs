#[cfg(feature = "python-bindings")]
use pyo3::prelude::*;

use super::lexer;

#[cfg_attr(feature = "python-bindings", pyclass(get_all))]
#[derive(PartialEq, Debug)]
pub struct Error {
    pub message: String,
    pub input_pos: usize,
}

pub struct Success<T> {
    value: T,
    remaining_input_string_start_index: usize,
}

pub type ParseResult<T> = Result<Success<T>, Error>;

pub trait Parser {
    fn parse<T>(&mut self) -> ParseResult<T>;
}

pub struct ParseState<'t, 'i> {
    lexer: lexer::Lexer<'t, 'i>,
    current_token: Option<lexer::Token<'t, 'i>>,
}

impl<'t, 'i> ParseState<'t, 'i> {
    pub fn new<'tt, 'ii>(lexer: lexer::Lexer<'tt, 'ii>) -> ParseState<'tt, 'ii> {
        ParseState {
            lexer,
            current_token: None,
        }
    }

    pub fn accept(
        &mut self,
        token_definition: &'t lexer::TokenDefinition,
    ) -> Option<lexer::Token<'t, 'i>> {
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

    pub fn expect_sub_parser<S>(&mut self, sub_parser: &mut impl Parser) -> ParseResult<S> {
        let parse_result = sub_parser.parse::<S>()?;

        self.lexer
            .advance_current_pos(parse_result.remaining_input_string_start_index);

        Ok(parse_result)
    }

    pub fn expect(
        &mut self,
        token_definition: &'t lexer::TokenDefinition,
    ) -> Result<lexer::Token<'t, 'i>, Error> {
        if self.current_token.is_none() {
            let option = self.lexer.get_next_token();

            if option.is_none() {
                let error_message = format!(
                    "Expected {descr} but there wasn't any next token.",
                    descr = token_definition.description
                );

                let input_pos = self.lexer.current_pos();

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
            input_pos: self.lexer.current_pos(),
        };

        return Err(error);
    }

    fn get_current_token(option: &Option<lexer::Token<'t, 'i>>) -> lexer::Token<'t, 'i> {
        option.as_ref().expect("Current token to be set.").clone()
    }
}
