from ffi_bridge import cpython_extension, pyo3_extension


def main() -> None:
    pyo3_person = pyo3_extension.PyPerson("some user", 23)
    print(pyo3_person)
    print(cpython_extension.greet_py_person())


if __name__ == '__main__':
    main()
