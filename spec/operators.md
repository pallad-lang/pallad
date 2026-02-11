# Operators Specification

## Overview / Purpose

This document defines the **operators currently implemented in Pallad**.  
Only core arithmetic and logical operators are included, along with their **syntax, precedence, and behavior**.  
Advanced or planned operators remain in `status.md`.

---

## Arithmetic Operators

| Operator | Example | Example result | Status | Notes |
| :------: | :-----: | :------------: | :----- | :---- |
| `+` | `x = 3 + 2` | `5` | Implemented | Addition; works for int, float |
| `-` | `x = 5 - 2` | `3` | Implemented | Subtraction; unary minus supported |
| `*` | `x = 4 * 2` | `8` | Implemented | Multiplication |
| `/` | `x = 5 / 2` | `2.5` | Implemented | Float division |
| `//` | `x = 5 // 2` | `2` | Implemented | Integer division |
| `%` | `x = 5 % 2` | `1` | Implemented | Modulus |
| `**` | `x = 2 ** 3` | `8` | Implemented | Exponentiation |

**Notes on precedence (highest → lowest):**

1. `**`
2. Unary `-`
3. `*`, `/`, `//`, `%`
4. `+`, `-`

---

## String Operators

| Operator | Example | Example result | Status | Notes |
| :------: | :-----: | :------------: | :----- | :---- |
| `+` (string concatenation) | `"Hello, " + "world!"` | `"Hellow, world!` | Implemented | Concatenates two strings |
| `*` (string repetition) | `"abc" * 3` | `abcabcabc` | Implemented | Repeats the string `n` times |

---

## Logical Operators

| Operator | Example | Example result | Status | Notes |
| :------: | :-----: | :------------: | :----- | :---- |
| `and` | `true and false` | `false` | Implemented | Planned short-circuit evaluation |
| `or` | `true or false` | `true` | Implemented | Planned short-circuit evaluation |
| `not` | `not true` | `false` | Implemented | Unary negation |

**Behavior Notes:**

- Logical operators return `bool`
- Evaluated left-to-right with short-circuiting
- `not` has highest logical precedence

---

## Notes / Status

- Only **implemented operators** are documented here.
- Planned operators (`==, !=, >, <, >=, <=, in, not in`, compound assignment, bitwise, ranges) remain in `status.md`.
- Operator precedence is consistent with standard mathematical conventions.
- Any changes in operator behavior must be synchronized with interpreter and bytecode.
