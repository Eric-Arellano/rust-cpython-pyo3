from ffi_bridge import cpython_extension, pyo3_extension


def main() -> None:
    pyo3_py_person = pyo3_extension.PyPerson("some user", 23)
    greeting = cpython_extension.greet_py_person(pyo3_py_person)
    print(greeting)


if __name__ == '__main__':
    main()
