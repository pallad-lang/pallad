# Syntax Specification

## Overviw / Propose

This document defines the syntax of the Pallad programming language.  
It describes valid tokens, literals, statements, and expressions that can appear in Pallad source code.  
Focus is on **features already implemented or nearly finalized**.

---

## Comments

### Single-line comments

```py
# This is a single-line comment
```

- **Status:** Implemented
- **Notes:**
  - Fully functional.
  - Extends to end-of-line.

### Multi-line comments

```py
"""
This is a
muti-line comment
"""
```

- **Status:** Implemented
- **Notes:**
  - Fully functional.
  - Can span multiple lines.

## Variables

### Declaration

```js
var x = 5
var name = "Alice"
```

- **Status:** Implemented
- **Notes:**
  - Duplicate cheching implemented (global).
  - Optional static typing planned.

### Optional static typing

```js
var x: int = 5
var name: string
```

- **Status:** Partial
- **Notes:**
  - Core type checking implemented.
  - Type inference in progress.

## Literals

| Type | Example | Status | Notes |
| :--- | :------ | :----- | :---- |
| Integer | `42` | Implemented | Standard decimal integers |
| Float | `3.14` | Implemented | Standard decimal floats |
| String | `"Hello"` | Implemented | Supports single-line and multi-line (`"""`) |
| Boolean | `true` / `false` | Implemented | Core boolean literals |
| None | `none` | Implemented | Represents null value |

## Expressions

### Arithmetic

```py
x = 5 + 3 * 2
y = (x - 2) / 3
```

- **Status:** Implemented
- **Notes:**
  - Operator precedence matches conventional math.
  - Power operator `**` supported.

### Logical

```js
flag = true and not false
```

- **Status:** Implemented
- **Notes:**
  - `and`, `or`, `not` implemented.
  - Short-circuit ecaluation planned.

---

## Notes / Status

- This file should **only contain implemented or nearly finalized syntax**.
- Any planned or partially implemented features should be referenced in `status.md`.
- Changes in syntax rules must be synchronized with interpreter and bytecode specification.
