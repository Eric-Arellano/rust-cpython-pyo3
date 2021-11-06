from ffi_bridge import pyo3_extension


def main() -> None:
    pyo3_person = pyo3_extension.PyPerson("some user", 23)
    print(pyo3_person)


if __name__ == '__main__':
    main()
