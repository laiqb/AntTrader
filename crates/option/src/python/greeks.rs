use pyo3::{
    IntoPyObjectExt,
    prelude::*,
    pyclass::CompareOp,
    types::{PyBytes, PyTuple},
};

use super::{IntoPyObjectNautilusExt, to_pyvalue_err};
use crate::greeks::*;
