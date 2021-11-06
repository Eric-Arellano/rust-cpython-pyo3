use pyo3::prelude::*;

#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: u32,
}

#[pyclass]
struct PyPerson(Person);

#[pymethods]
impl PyPerson {
    #[new]
    fn __new__(name: String, age: u32) -> PyPerson {
        PyPerson(Person { name, age })
    }

    fn __repr__(&self) -> String {
        format!("PyPerson(name='{}', age={})", self.0.name, self.0.age)
    }
}

#[pymodule]
fn pyo3_extension(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPerson>()?;
    Ok(())
}
