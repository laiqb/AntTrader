use std::fmt::{Debug, Display, Formatter};

use ant_core::correctness::{
    FAILED,
    check_string_contains,
    check_valid_string,
};

use ustr::Ustr;

/// Represents a valid trader ID.
#[repr(C)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "ant_trader.core.ant_pyo3.model")
)]
pub struct TraderId(Ustr);

impl TraderId{
    pub fn new_checked<T: AsRef<str>>(value: T) -> anyhow::Result<Self>{
        let value = value.as_ref();
        check_valid_string(value, stringify!(value))?;
        check_string_contains(value, "-", stringify!(value))?;
        Ok(Self(Ustr::from(value)))
    }

    pub fn new<T: AsRef<str>>(value: T) -> Self{
        Self::new_checked(value).expect(FAILED)
    }

    #[allow(dead_code)]
    pub(crate) fn set_inner(&mut self, value:&str){
        self.0 = Ustr::from(value);
    }

    #[must_use]
    pub fn inner(&self) -> Ustr{
        self.0
    }

    #[must_use]
    pub fn as_str(&self) -> &str{
        self.0.as_str()
    }

    #[must_use]
    pub fn get_tag(&self) -> &str {
        // SAFETY: Unwrap safe as value previously validated
        self.0.split('-').next_back().unwrap()
    }
}

impl Debug for TraderId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for TraderId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}