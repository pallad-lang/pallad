# Pallad Semantics

## Overview

This document describes the **runtime behavior** and **meaning** of Pallad constructs.

---

## Expression Evaluation

- Arithmetic operators: `+`, `-`, `*`, `/`, `//`, `%`, `**`
- Logical operators: `and`, `or`, `not`
- Operator precedence follows conventional rules (planned: document full table)
- Unary operators: `+`, `-` (`+` not supported yet)
- Expression evaluation produces a value according to type rules

---

## Built-in Functions

### `print`

- Accepts **any number of arguments**.
- Each argument is printed **on a new line**.
- LF is automatically appended after each argument.
- Supports all currently implemented types (`int`, `float`, `bool`, `none`, `string`).

---

## Variables

- Declared using `var` keyword.
- Optional static typing with `var x: int`. (planned)
- Scope: (planned)
  - Variables default to **local scope**.
  - Global variables must be explicitly declared.
- Type errors are detected **at runtime** unless static typing is used. (planned)

---

## Control Flow

- Conditional statements: `if`, `elif`, `else` (planned)
- Pattern matching: `match` (planned)
- Loops: `for`, `while`, `do-while` (planned)
- Loop control: `break`, `continue` (planned)
- Exception handling: `try`, `except`, `else`, `ensure`, `raise`, `assert` (planned)

---

## Object-Oriented Semantics (Planned)

- Class creation: `class`, inheritance via `extends`
- Static members: `static var`, `static func`
- Inner classes supported in future
- Constructors: default, static, custom, copy, overloading
- Method resolution order and attribute lookup rules
- Imports: `import ...`, `exclude ...`, `... as ...`, `from ... import ...`

---

## Notes

- This is an **initial draft** reflecting features implemented or partially implemented.
- Future updates will refine:
  - Operator precedence table
  - Type coercion and mixing rules
  - Full OOP behavior and method resolution
  - Exception semantics
  - Advanced control flow features
