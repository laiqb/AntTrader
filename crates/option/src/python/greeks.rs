use pyo3::{
    IntoPyObjectExt,
    prelude::*,
    pyclass::CompareOp,
    types::{PyBytes, PyTuple},
};
use crate::greeks::*;

#[pymethods]
impl ImplyVolAndGreeksResult {
    #[new]
    fn py_new(vol: f64,
              price: f64,
              delta: f64,
              gamma: f64,
              vega: f64,
              theta: f64) -> Self {
        Self{vol, price, delta, gamma, vega, theta}
    }

    #[getter]
    #[pyo3(name = "vol")]
    fn get_vol(&self) -> f64 {
        self.vol
    }

    #[getter]
    #[pyo3(name = "price")]
    fn get_price(&self) -> f64 {
        self.price
    }

    #[getter]
    #[pyo3(name = "delta")]
    fn get_delta(&self) -> f64 {
        self.delta
    }

    #[getter]
    #[pyo3(name = "gamma")]
    fn get_gamma(&self) -> f64 {
        self.gamma
    }

    #[getter]
    #[pyo3(name = "vega")]
    fn get_vega(&self) -> f64 {
        self.vega
    }

    #[getter]
    #[pyo3(name = "theta")]
    fn get_theta(&self) -> f64 {
        self.theta
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[pymethods]
impl BlackScholesGreeksResult {
    #[new]
    fn py_new(price: f64, delta: f64, gamma: f64, vega: f64, theta: f64) -> Self {
        Self{price, delta, gamma, vega, theta}
    }

    #[getter]
    #[pyo3(name = "price")]
    fn get_price(&self) -> f64 {
        self.price
    }

    #[getter]
    #[pyo3(name = "delta")]
    fn get_delta(&self) -> f64 {
        self.delta
    }

    #[getter]
    #[pyo3(name = "gamma")]
    fn get_gamma(&self) -> f64 {
        self.gamma
    }

    #[getter]
    #[pyo3(name = "vega")]
    fn get_vega(&self) -> f64 {
        self.vega
    }

    #[getter]
    #[pyo3(name = "theta")]
    fn get_theta(&self) -> f64 {
        self.theta
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[must_use]
#[pyfunction(name = "black_scholes_greeks")]
pub fn py_black_scholes_greeks(under_type: i32,
                         s: f64,
                         r: f64,
                         b: f64,
                         sigma: f64,
                         is_call: bool,
                         k: f64,
                         t: f64,
                         multiplier: f64) -> PyResult<BlackScholesGreeksResult> {
    let result = black_scholes_greeks(under_type, s, r, b, sigma, is_call, k, t, multiplier);
    Ok(result)
}

#[must_use]
#[pyfunction(name = "imply_vol")]
pub fn py_imply_vol(under_type: i32,
                    price: f64,
                    s: f64,
                    r: f64,
                    b: f64,
                    is_call: bool,
                    k: f64,
                    t: f64,
                    multiplier: f64) -> PyResult<ImplyVolAndGreeksResult> {
    let result = imply_vol(under_type, price, s, r, b, is_call, k, t, multiplier);
    Ok(result)
}