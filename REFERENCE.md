# Pallad Language Reference

This document serves as a concise reference guide for the Pallad programming language. It provides essential details to help programmers get started without delving into unnecessary complexity. The focus is primarily on language syntax and core features.

## Quick Start
Create a new file with the `.pd` extension (for example, `first.pd`). For syntax highlighting, you may select Python in your editor. While not fully accurate, it provides reasonable coverage since Pallad syntax is similar to Python. Pallad does not yet have a dedicated highlighter.

Example program:

```python
print("Hello, World!")
```

To execute the file, pass it as an argument to the Pallad interpreter:

```shell
pallad first.pd
```

## Lexical Structure
### Comments
Comments in Pallad begin with the hash (`#`) character and continue to the end of the line. Multi-line comments are not currently supported; use multiple single-line comments instead.

```python
# Hello World!
# This is a comment in Pallad
```

### Identifiers
Identifiers are names used for variables, functions, and objects. They may contain letters, digits, or underscores (`_`), but cannot begin with a digit. Identifiers are case-sensitive. Identifiers starting with `_` are considered private; others are public.

- Variables and functions: `snake_case` is recommended.  
- Objects: `PascalCase` is preferred.

### Whitespace and New Lines
Pallad is indentation-based. Tabs (`\t`) with a width of four spaces are the preferred indentation style. Statements are typically terminated by a newline, but can be grouped with parentheses (`(`, `)`) to span multiple lines.

Whitespace between tokens is ignored, though proper spacing is recommended for readability.

> **Warning**  
> Multi-line statements using parentheses are not yet supported.

## Type System
Pallad is dynamically typed, meaning types are determined at runtime rather than at compile time.

> **Tip**  
> Pallad compiles to bytecode and is then interpreted in Rust. “Compile time” refers to the bytecode generation phase.

Currently implemented types:
- **`float`**: Floating-point numbers  
- **`int`**: Integer numbers  
- **`string`**: Text enclosed in `""` or `''`

> **Important**  
> Most Pallad types are not yet implemented. The following list represents planned types (subject to change):

| Type                    | Keyword   | Type       | Keyword   |
|--------------------------|-----------|------------|-----------|
| Variant                 | `variant` | Void       | `void`    |
| Boolean                 | `bool`    | Integer    | `int`     |
| Byte                    | `byte`    | Char       | `char`    |
| State                   | `state`   | Float      | `float`   |
| Complex                 | `complex` | Array      | `array`   |
| Multi-dimensional Array | `mdarray` | Table      | `table`   |
| Queue                   | `queue`   | Buffer     | `buffer`  |
| Set                     | `set`     | Tuple      | `tuple`   |
| Pair                    | `pair`    | Triplet    | `triplet` |
| Dictionary              | `dict`    | String     | `string`  |
| Callable                | `function`| Color      | `color`   |

### Truthiness
In boolean contexts (such as `if` or `while` conditions), non-boolean values are converted to boolean according to the following rules:

| Type     | Evaluates to `false` |
|----------|-----------------------|
| `null`   | `null` (always false) |
| `bool`   | `false`               |
| `int`    | `0`                   |
| `float`  | `0.0`                 |
| `string` | `""` (empty string)   |

All other values are considered `true`.

> **Note**  
> `null` and `bool` types are not yet implemented. This section describes planned behavior.

---

This file is incomplete due to unimplemented features. Future updates will include:

- Detailed type descriptions (`null`, `bool`, `int`, `float`, `string`, etc.)  
- Static typing and runtime type checking  
- Type conversions  
- Grammar and expression rules (arithmetic, boolean, assignment, grouping)  
- Statement definitions (`if`, `match`, `for`, `while`, `try/catch/finally`, `raise`, `var`, `func`, `object`, `import`, etc.)  
- Standard library functions and modules  
