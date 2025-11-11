pub mod greeks;

use pyo3::prelude::*;
#[pymodule]
pub fn python(_:Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {

    m.add_class::<crate::greeks::ImplyVolAndGreeksResult>()?;
    m.add_class::<crate::greeks::BlackScholesGreeksResult>()?;

    m.add_function(wrap_pyfunction!(
        crate::python::greeks::py_black_scholes_greeks,
        m
    )?)?;

    m.add_function(wrap_pyfunction!(
        crate::python::greeks::py_imply_vol,
        m
    )?)?;
    Ok(())
}
