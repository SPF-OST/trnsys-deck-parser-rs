use super::base::{lexer, parser};

struct Parser<'t, 'i> {
    state: parser::ParseState<'t, 'i>,
}

struct Ddck {}

impl<'t, 'i> parser::Parser for Parser<'t, 'i> {
    fn parse<Ddck>(&mut self) -> parser::ParseResult<Ddck> {
        Err(parser::Error {
            message: "Not implemented!".to_string(),
            input_pos: 0,
        })
    }
}

impl<'t, 'i> Parser<'t, 'i> {
    pub fn new<'tt, 'ii>(lexer: lexer::Lexer<'tt, 'ii>) -> Parser<'tt, 'ii> {
        let parse_state = parser::ParseState::new(lexer);

        Parser { state: parse_state }
    }
}
