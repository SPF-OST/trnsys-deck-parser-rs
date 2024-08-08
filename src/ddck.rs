#[cfg(feature = "python-bindings")]
use std::hash::{DefaultHasher, Hash, Hasher};

#[cfg(feature = "python-bindings")]
use pyo3::prelude::*;

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
