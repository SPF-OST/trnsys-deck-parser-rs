#[cfg(feature = "python-bindings")]
use std::hash::{DefaultHasher, Hash, Hasher};

#[cfg(feature = "python-bindings")]
use pyo3::prelude::*;

struct TokenDefinitions {
    variable: TokenDefinition,
}

impl TokenDefinitions {
    fn new() -> TokenDefinitions {
        TokenDefinitions {
            variable: TokenDefinition::new("variable name", r"[A-Za-z]+"),
        }
    }
}

struct TokenDefinition {
    description: String,
    regex: regex::Regex,
    #[allow(dead_code)]
    priority: i32,
}

impl TokenDefinition {
    fn new_custom(
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

    fn new(description: &str, regex_pattern: &str) -> TokenDefinition {
        let case_sensitive = false;
        let priority = -1;

        Self::new_custom(description, regex_pattern, case_sensitive, priority)
    }
}

pub struct Lexer<'a> {
    input: String,
    current_pos: usize,
    token_definitions: Vec<&'a TokenDefinition>,
}

#[cfg_attr(feature = "python-bindings", pyclass)]
#[derive(PartialEq, Debug)]
pub struct Error {
    pub message: String,
}

impl Lexer<'_> {
    pub fn new<'a>(
        input: String,
        current_pos: usize,
        token_definitions: Vec<&'a TokenDefinition>,
    ) -> Lexer {
        Lexer {
            input,
            current_pos,
            token_definitions,
        }
    }

    pub fn get_next_token(&mut self) -> Option<&TokenDefinition> {
        for token_definition in &self.token_definitions {
            match (&token_definition.regex).find(&self.input) {
                Some(m) => {
                    self.current_pos = m.end();
                    return Some(&token_definition);
                }
                None => (),
            }
        }

        return None;
    }
}

pub struct ParserBase<'a> {
    lexer: &'a Lexer<'a>,
    current_token: &'a TokenDefinition,
}

impl ParserBase<'_> {
    pub fn accept(_token_definition: &TokenDefinition) {}
}

#[cfg_attr(feature = "python-bindings", pyclass(get_all))]
#[derive(PartialEq, Debug)]
pub struct Variable {
    pub name: String,
}

#[cfg(feature = "python-bindings")]
#[pymethods]
impl Variable {
    #[new]
    pub fn new(name: &str) -> Variable {
        Variable {
            name: String::from(name),
        }
    }

    pub fn __hash__(&self) -> isize {
        let mut hasher = DefaultHasher::new();
        self.name.hash(&mut hasher);
        hasher.finish().try_into().unwrap()
    }

    pub fn __eq__(&self, other: &Self) -> bool {
        return self == other;
    }
}

#[cfg(not(feature = "python-bindings"))]
impl Variable {
    pub fn new(name: &str) -> Variable {
        Variable {
            name: String::from(name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_variable() {
        let token_definitions = TokenDefinitions::new();
        let lexer = Lexer::new(
            "VIceSscaled".to_string(),
            0,
            [&token_definitions.variable].to_vec(),
        );
    }

    #[test]
    fn test_variables_equal() {
        let var1 = Variable::new("VIceSscaled");
        let var2 = Variable::new("VIceSscaled");
        assert_eq!(var1, var2)
    }

    #[test]
    fn test_variables_not_equal() {
        let var1 = Variable::new("AColl");
        let var2 = Variable::new("VIceSscaled");
        assert_ne!(var1, var2)
    }
}
