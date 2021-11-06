use pyo3::prelude::*;

#[derive(Clone, Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

#[pyclass]
pub struct PyPerson(pub Person);

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
