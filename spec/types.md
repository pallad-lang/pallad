# Pallad Types

## Overview

This document describes **all data types** currently available or planned in Pallad, their properties, and how they interact with the type system.

---

## Primitive Types

| Type | Status | Notes |
| :--- | :----- | :---- |
| `int` | 🟢Implemented | Standard integers |
| `float` | 🟢Implemented | Floating-point numbers |
| `bool` | 🟢Implemented | Boolean values (`true` / `false`) |
| `none` | 🟢Implemented | Represents absence of value |
| `string` | 🟡Partial | Single-line and multi-line supported; concatenation and repetition implemented |

---

## Compound / Advanced Types (Planned)

| Type | Status | Notes |
| :--- | :----- | :---- |
| `array` | 🔵Planned | Homogeneous collections |
| `mdarray` | 🔵Planned | Multi-dimensional arrays |
| `table` | 🔵Planned | Key-value table structures |
| `queue` | 🔵Planned | FIFO queues |
| `buffer` | 🔵Planned | Byte buffer |
| `set` | 🔵Planned | Unique collections |
| `tuple` | 🔵Planned | Ordered collections of fixed size |
| `pair` | 🔵Planned | 2-element tuple |
| `triplet` | 🔵Planned | 3-element tuple |
| `dict` | 🔵Planned | Key-value mapping |
| `variant` | 🔵Planned | Dynamic type holder |
| `function` | 🔵Planned | Function type for callable objects |
| `struct` / `record` | 🔵Planned | User-defined composite types |
| Type mixing | 🔵Planned | Mixing types in expressions |

---

## Type System

- Pallad supports **hybrid typing**:
  - **Dynamic typing** by default
  - **Optional static typing** (`var x: int`) (planned)
- Type inference is **partial** and currently being expanded.
- Type errors are **runtime errors** unless static typing is enforced. (planned for compile-time errors)
- Operations between incompatible types raise errors unless type coercion rules are defined in future versions.
