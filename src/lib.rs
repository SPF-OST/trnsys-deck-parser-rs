mod parse {
    use regex;

    struct Tokens {
        variable: regex::Regex,
    }

    impl Tokens {
        pub fn new() -> Tokens {
            Tokens {
                variable: regex::Regex::new(r"[A-Za-z]+").unwrap(),
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct Variable<'i> {
        name: &'i str,
    }

    #[derive(PartialEq, Debug)]
    pub struct Error<'i> {
        message: &'i str,
    }

    fn parse_variable<'i, 'e>(tokens: Tokens, input: &'i str) -> Result<Variable<'i>, Error<'e>> {
        match tokens.variable.find(&input) {
            Some(m) => Ok(Variable { name: m.as_str() }),
            None => Err(Error {
                message: "Not a variable.",
            }),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_a_variable() {
            let tokens = Tokens::new();

            let result = parse_variable(tokens, "VIceSscaled");

            assert_eq!(
                result,
                Ok(Variable {
                    name: "VIceSscaled"
                })
            );
        }

        #[test]
        fn test_not_a_variable() {
            let tokens = Tokens::new();

            let result = parse_variable(tokens, "1.297");

            assert_eq!(
                result,
                Err(Error {
                    message: "Not a variable."
                })
            );
        }
    }
}
