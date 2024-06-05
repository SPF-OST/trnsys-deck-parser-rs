use pyo3::create_exception;
use pyo3::exceptions::PyException;

create_exception!(trnsys_deck_parser_rs, ParseError, PyException);

impl From<super::parse::base::Error> for pyo3::PyErr {
    fn from(error: super::parse::base::Error) -> Self {
        ParseError::new_err(error.message)
    }
}
