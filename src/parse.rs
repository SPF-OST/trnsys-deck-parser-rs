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

#[cfg_attr(feature = "python-bindings", pyclass)]
pub struct Parser {
    token_definitions: TokenDefinitions,
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

#[cfg_attr(feature = "python-bindings", pyclass)]
#[derive(PartialEq, Debug)]
pub struct Error {
    pub message: String,
}

#[cfg_attr(feature = "python-bindings", pymethods)]
impl Parser {
    #[cfg(feature = "python-bindings")]
    #[new]
    pub fn new() -> Parser {
        Parser {
            token_definitions: TokenDefinitions::new(),
        }
    }

    pub fn parse_variable(&self, input: &str) -> Result<Variable, Error> {
        let token_definition = &self.token_definitions.variable;

        match (&token_definition.regex).find(input) {
            Some(m) => {
                let name = String::from(m.as_str());
                Ok(Variable { name })
            }
            None => {
                let message = format!("Not a {}.", token_definition.description);
                Err(Error { message })
            }
        }
    }
}

#[cfg(not(feature = "python-bindings"))]
impl Parser {
    pub fn new() -> Parser {
        Parser {
            token_definitions: TokenDefinitions::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_variable() {
        let parser = Parser::new();

        let result = parser.parse_variable("VIceSscaled");

        assert_eq!(
            result,
            Ok(Variable {
                name: String::from("VIceSscaled")
            })
        );
    }

    #[test]
    fn test_not_a_variable() {
        let parser = Parser::new();

        let result = parser.parse_variable("1.297");

        assert_eq!(
            result,
            Err(Error {
                message: String::from("Not a variable name.")
            })
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
