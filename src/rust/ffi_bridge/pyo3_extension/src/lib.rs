use pyo3::prelude::*;
use pyo3::{AsPyPointer, FromPyPointer};
use pyo3_lib::PyPerson;

#[pymodule]
fn pyo3_extension(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<pyo3_lib::PyPerson>()?;
    m.add_function(wrap_pyfunction!(greet_py_person, m)?)?;
    Ok(())
}

fn from_pyo3_pointer_to_pyo3<'py>(value: &PyAny, py: Python<'py>) -> &'py PyAny {
    // NB: Gives TypeError when using `from_shared_ptr` instead of `from_owned_ptr`.
    unsafe { pyo3::PyAny::from_owned_ptr(py, value.as_ptr()) }
}

#[pyfunction]
fn greet_py_person(py_person_ptr: &PyAny) -> String {
    let py_person: PyPerson =
        Python::with_gil(|py| from_pyo3_pointer_to_pyo3(py_person_ptr, py).extract()).unwrap();
    format!(
        "Hello, {}! You are {} years old, cool!",
        py_person.0.name, py_person.0.age
    )
}
