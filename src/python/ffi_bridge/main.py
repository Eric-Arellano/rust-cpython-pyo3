from ffi_bridge import cpython_extension, pyo3_extension


def main() -> None:
    pyo3_py_person = pyo3_extension.PyPerson("some user", 23)
    pyo3_greeting = pyo3_extension.greet_py_person(pyo3_py_person)
    print(pyo3_greeting)

    # Issue #1: this renders `write` instead of the `PyPerson`. It's like `greet_by_person`
    # corrupted the object.
    print(pyo3_py_person)

    # Issue #2: If you comment out the lines 6-7, this crashes with TypeError.
    cpython_greeting = cpython_extension.greet_py_person(pyo3_py_person)
    print(cpython_greeting)


if __name__ == '__main__':
    main()
