use cpython::{py_fn, py_module_initializer, PyResult, Python};
use pyo3_lib::{Person, PyPerson};

py_module_initializer!(cpython_extension, |py, m| {
    #[allow(clippy::manual_strip)]
    m.add(py, "greet_py_person", py_fn!(py, greet_py_person()))?;
    Ok(())
});

fn greet_py_person(_: Python) -> PyResult<String> {
    // TODO: read a PyPerson from FFI pointer.
    let py_person = PyPerson(Person {
        name: "hardcoded".to_owned(),
        age: 23,
    });
    Ok(format!(
        "Hello, {}! You are {} years old, cool!",
        py_person.0.name, py_person.0.age
    ))
}
