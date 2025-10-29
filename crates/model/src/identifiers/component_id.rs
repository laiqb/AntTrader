
use std::{
    fmt::{Debug, Display, Formatter},
    hash::Hash,
};

use ant_core::correctness::{FAILED, check_valid_string};
use ustr::Ustr;

#[repr(C)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "ant_trader.core.ant_pyo3.model")
)]
pub struct ComponentId(Ustr);


impl ComponentId {
    /// Creates a new [`ComponentId`] instance with correctness checking.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is not a valid string.
    ///
    /// # Notes
    ///
    /// PyO3 requires a `Result` type for proper error handling and stacktrace printing in Python.
    pub fn new_checked<T: AsRef<str>>(value: T) -> anyhow::Result<Self> {
        let value = value.as_ref();
        check_valid_string(value, stringify!(value))?;
        Ok(Self(Ustr::from(value)))
    }

    /// Creates a new [`ComponentId`] instance.
    ///
    /// # Panics
    ///
    /// Panics if `value` is not a valid string.
    pub fn new<T: AsRef<str>>(value: T) -> Self {
        Self::new_checked(value).expect(FAILED)
    }

    /// Sets the inner identifier value.
    #[allow(dead_code)]
    pub(crate) fn set_inner(&mut self, value: &str) {
        self.0 = Ustr::from(value);
    }

    /// Returns the inner identifier value.
    #[must_use]
    pub fn inner(&self) -> Ustr {
        self.0
    }

    /// Returns the inner identifier value as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Debug for ComponentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for ComponentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::ComponentId;
    use crate::identifiers::stubs::*;

    #[rstest]
    fn test_string_reprs(component_risk_engine: ComponentId) {
        assert_eq!(component_risk_engine.as_str(), "RiskEngine");
        assert_eq!(format!("{component_risk_engine}"), "RiskEngine");
    }
}
