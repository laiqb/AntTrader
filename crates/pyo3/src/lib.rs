
#![warn(rustc::all)]
#![deny(unsafe_code)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(rustdoc::broken_intra_doc_links)]

use std::path::Path;

use pyo3::prelude::*;


/// We modify sys modules so that submodule can be loaded directly as
/// import supermodule.submodule
///
/// Also re-exports all submodule attributes so they can be imported directly from `ant_pyo3`
/// refer: <https://github.com/PyO3/pyo3/issues/2644>
#[pymodule] // The name of the function must match `lib.name` in `Cargo.toml`
#[cfg_attr(feature = "cython-compat", pyo3(name = "ant_pyo3"))]
fn _libant(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let sys = PyModule::import(py, "sys")?;
    let modules = sys.getattr("modules")?;
    let sys_modules: &Bound<'_, PyAny> = modules.downcast()?;

    #[cfg(feature = "cython-compat")]
    let module_name = "ant_trader.core.ant_pyo3";

    #[cfg(not(feature = "cython-compat"))]
    let module_name = "ant_trader._libant";

    // Set pyo3_ant to be recognized as a subpackage
    sys_modules.set_item(module_name, m)?;

    let n = "option";
    let submodule = pyo3::wrap_pymodule!(ant_option::python::option);
    m.add_wrapped(submodule)?;
    sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    #[cfg(feature = "cython-compat")]
    re_export_module_attributes(m, n)?;

    // let n = "core";
    // let submodule = pyo3::wrap_pymodule!(ant_core::python::core);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "common";
    // let submodule = pyo3::wrap_pymodule!(ant_common::python::common);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "cryptography";
    // let submodule = pyo3::wrap_pymodule!(ant_cryptography::python::cryptography);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "indicators";
    // let submodule = pyo3::wrap_pymodule!(ant_indicators::python::indicators);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "infrastructure";
    // let submodule = pyo3::wrap_pymodule!(ant_infrastructure::python::infrastructure);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "live";
    // let submodule = pyo3::wrap_pymodule!(ant_live::python::live);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "model";
    // let submodule = pyo3::wrap_pymodule!(ant_model::python::model);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "network";
    // let submodule = pyo3::wrap_pymodule!(aant_network::python::network);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "persistence";
    // let submodule = pyo3::wrap_pymodule!(ant_persistence::python::persistence);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "serialization";
    // let submodule = pyo3::wrap_pymodule!(ant_serialization::python::serialization);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "testkit";
    // let submodule = pyo3::wrap_pymodule!(ant_testkit::python::testkit);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "trading";
    // let submodule = pyo3::wrap_pymodule!(ant_trading::python::trading);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // // Adapters
    //
    // let n = "bitmex";
    // let submodule = pyo3::wrap_pymodule!(ant_bitmex::python::bitmex);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "coinbase_intx";
    // let submodule = pyo3::wrap_pymodule!(ant_coinbase_intx::python::coinbase_intx);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "databento";
    // let submodule = pyo3::wrap_pymodule!(ant_databento::python::databento);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "hyperliquid";
    // let submodule = pyo3::wrap_pymodule!(ant_hyperliquid::python::hyperliquid);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "okx";
    // let submodule = pyo3::wrap_pymodule!(ant_okx::python::okx);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;
    //
    // let n = "tardis";
    // let submodule = pyo3::wrap_pymodule!(ant_tardis::python::tardis);
    // m.add_wrapped(submodule)?;
    // sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
    // #[cfg(feature = "cython-compat")]
    // re_export_module_attributes(m, n)?;

    #[cfg(feature = "defi")]
    {
        let n = "blockchain";
        let submodule = pyo3::wrap_pymodule!(ant_blockchain::python::blockchain);
        m.add_wrapped(submodule)?;
        sys_modules.set_item(format!("{module_name}.{n}"), m.getattr(n)?)?;
        #[cfg(feature = "cython-compat")]
        re_export_module_attributes(m, n)?;
    }

    Ok(())
}

#[cfg(feature = "cython-compat")]
fn re_export_module_attributes(
    parent_module: &Bound<'_, PyModule>,
    submodule_name: &str,
) -> PyResult<()> {
    let submodule = parent_module.getattr(submodule_name)?;
    for item_name in submodule.dir()? {
        let item_name_str: &str = item_name.extract()?;
        if let Ok(attr) = submodule.getattr(item_name_str) {
            parent_module.add(item_name_str, attr)?;
        }
    }

    Ok(())
}

/// Generate Python type stub info for PyO3 bindings.
///
/// Assumes the pyproject.toml is located in the python/ directory relative to the workspace root.
///
/// # Panics
///
/// Panics if the path locating the pyproject.toml is incorrect.
///
/// # Errors
///
/// Returns an error if stub information generation fails.
///
/// # Reference
///
/// - <https://pyo3.rs/latest/python-typing-hints>
/// - <https://crates.io/crates/pyo3-stub-gen>
/// - <https://github.com/Jij-Inc/pyo3-stub-gen>
pub fn stub_info() -> pyo3_stub_gen::Result<pyo3_stub_gen::StubInfo> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let pyproject_path = workspace_root.join("python").join("pyproject.toml");

    pyo3_stub_gen::StubInfo::from_pyproject_toml(&pyproject_path)
}
