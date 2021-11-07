use cpython::{py_fn, py_module_initializer};
use pyo3::FromPyPointer;
use pyo3_lib::PyPerson;

py_module_initializer!(cpython_extension, |py, m| {
    #[allow(clippy::manual_strip)]
    m.add(
        py,
        "greet_py_person",
        py_fn!(py, greet_py_person(a: cpython::PyObject)),
    )?;
    Ok(())
});

fn from_rust_cpython_to_pyo3(value: cpython::PyObject, py: pyo3::Python) -> &pyo3::PyAny {
    unsafe { pyo3::PyAny::from_owned_ptr(py, value.as_ptr() as *mut pyo3::ffi::PyObject) }
}

fn greet_py_person(
    _: cpython::Python,
    py_person_ptr: cpython::PyObject,
) -> cpython::PyResult<String> {
    let py_person: PyPerson =
        pyo3::Python::with_gil(|py| from_rust_cpython_to_pyo3(py_person_ptr, py).extract())
            .unwrap();
    Ok(format!(
        "Hello, {}! You are {} years old, cool!",
        py_person.0.name, py_person.0.age
    ))
}
