#[derive(Debug)]
pub struct TokenDefinition {
    pub description: String,
    regex: regex::Regex,
    #[allow(dead_code)]
    priority: i32,
}

#[derive(Clone)]
pub struct Token<'t, 'i> {
    pub definition: &'t TokenDefinition,
    value: &'i str,
}

impl TokenDefinition {
    pub fn new_custom(
        description: &str,
        regex_pattern: &str,
        case_sensitive: bool,
        priority: i32,
    ) -> TokenDefinition {
        let regex_pattern_with_start_anchor = format!(r"\A{regex_pattern}");
        let mut builder = regex::RegexBuilder::new(&regex_pattern_with_start_anchor);
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

impl<'t, 'i> Lexer<'t, 'i> {
    pub fn new<'ii, 'tt, I>(
        input: &'ii str,
        current_pos: usize,
        token_definitions: I,
    ) -> Lexer<'tt, 'ii> 
    where I : Iterator<Item = &'tt TokenDefinition>
    {
        Lexer {
            input,
            current_pos,
            token_definitions: Vec::from_iter(token_definitions),
        }
    }

    pub fn current_pos(&self) -> usize {
        self.current_pos
    }

    pub fn get_next_token<'l>(&'l mut self) -> Option<Token<'t, 'i>> {
        for token_definition in &self.token_definitions {
            match (token_definition.regex).find_at(&self.input, self.current_pos) {
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

    pub fn advance_current_pos(&mut self, new_current_pos: usize) {
        if new_current_pos < new_current_pos {
            panic!("The new position {new_current_pos} must be greater than or equal to the current position {current_pos}",
                    new_current_pos=new_current_pos,
                    current_pos=self.current_pos);
        }

        self.current_pos = new_current_pos;
    }
}
