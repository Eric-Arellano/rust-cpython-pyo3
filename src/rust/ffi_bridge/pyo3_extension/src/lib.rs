use pyo3::prelude::*;

#[pymodule]
fn pyo3_extension(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<pyo3_lib::PyPerson>()?;
    Ok(())
}
