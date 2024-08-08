use super::base;
use super::lexer;

struct Parser<'t, 'i> {
    token_definitions: &'t lexer::TokenDefinitions,
    state: base::parser::ParseState<'t, 'i>,
}

#[derive(Debug)]
struct Ddck {}

impl<'t, 'i> base::parser::Parser<Ddck> for Parser<'t, 'i> {
    fn parse(&mut self) -> base::parser::ParseResult<Ddck> {
        Err(base::parser::Error {
            message: "Not implemented!".to_string(),
            input_pos: 0,
        })
    }
}

impl<'t, 'i> Parser<'t, 'i> {
    pub fn new<'tt, 'ii>(
        token_definitions: &'tt lexer::TokenDefinitions,
        input: &'ii str,
    ) -> Parser<'tt, 'ii> {
        let lexer = base::lexer::Lexer::new(input, 0, token_definitions.all().into_iter());
        let parse_state = base::parser::ParseState::new(lexer);

        Parser {
            token_definitions,
            state: parse_state,
        }
    }
}

#[cfg(test)]
mod tests {
    use base::parser::Parser;

    use super::*;

    #[test]
    fn test_parse_not_implemented() {
        let token_definitions = lexer::TokenDefinitions::new();

        let input = "EQUATIONS 1
VIceS = 4
";
        let mut parser = super::Parser::new(&token_definitions, input);

        parser.parse().expect_err("Shouldn't be implemented yet.");
    }
}
