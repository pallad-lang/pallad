# Built-ins Specification

## Overview / Purpose

This document defines the **built-in functions** available in Pallad.  
Currently, only the `print` function is fully implemented.  

---

## print

### Syntax

```py
print(value1, value2, ..., valueN)
```

- **Status:** Implemented
- **Notes:**
  - `print` takes **any number of arguments**.
  - Each argument is printed on a **new line**.
  - After printing all arguments, a newline is automatically added at the end of the last argument.
  - Supports any data type (strings, numbers, etc.).

### Example

```py
print("Hello,")
print("Pallad!")
print(42)
print(3.14)
```

- **Output:**

  ```text
  Hello,
  Pallad!
  42
  3.14
  ```

---

## Notes / Status

- **`print`** is currently the only fully implemented built-in function.
- Future built-ins (`input`, `range`, `call`, etc.) are planned and tracked in `status.md`.
- Additional formatting options (custom separator, end character) may be added in future versions.
