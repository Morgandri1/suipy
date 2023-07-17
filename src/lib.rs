use pyo3::prelude::*;

pub mod objects;
pub mod send;

#[pymodule]
fn suipy(_: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(objects::getOwnedObjects, m)?)?;
    m.add_function(wrap_pyfunction!(send::sendSui, m)?)?;
    Ok(())
}