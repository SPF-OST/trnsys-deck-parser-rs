mod parse;

#[cfg(feature = "python-bindings")]
mod python;

#[cfg(feature = "python-bindings")]
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[cfg(feature = "python-bindings")]
#[pymodule]
fn trnsys_deck_parser_rs(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<parse::Variable>()?;
    m.add_class::<parse::Parser>()?;
    m.add("ParseError", py.get_type::<python::ParseError>())?;
    Ok(())
}
