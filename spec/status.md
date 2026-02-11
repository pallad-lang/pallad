# Pallad Feature Status

This document lists **all Pallad features** with their current implementation status.  
Sections are grouped by category for readability.

> Status options: `🟢Implemented`, `🟡Partial`, `🔵Planned`, `🔴Non-Goal`

---

## Comments

| Feature | Status | Notes |
| :------ | :----- | :---- |
| Single-line comments `#` | 🟢Implemented | - |
| Multi-line comments `"""` | 🟢Implemented | - |
| BBCode converter module | 🔵Planned | Idea stage |
| Highlight rules for single-line comments | 🔵Planned | To implement color coding for `FIXME`, `NOTE`, etc. |
| Folding rules (`#region ... #endregion`) | 🔵Planned | - |

---

## Constants

| Feature | Status | Notes |
| :------ | :----- | :---- |
| `const` keyword | 🔵Planned | - |
| Invalid `none` constants | 🔵Planned | - |

---

## Variables

| Feature | Status | Notes |
| :------ | :----- | :---- |
| `var` keyword | 🟢Implemented | - |
| Optional static typing | 🟡Partial | Type inference in progress |
| Auto-type with `:=` | 🔵Planned | - |
| `setter()` / `getter()` | 🔵Planned | - |

---

## Data Types

| Feature | Status | Notes |
| :------ | :----- | :---- |
| `variant` | 🔵Planned | - |
| `void` | 🔵Planned | - |
| `none` | 🟢Implemented | - |
| `bool` | 🟢Implemented | - |
| `int` | 🟢Implemented | - |
| `byte` | 🔵Planned | - |
| `char` | 🔵Planned | - |
| `state` | 🔵Planned | - |
| `float` | 🟢Implemented | - |
| `complex` | 🔵Planned | - |
| `array` | 🔵Planned | - |
| `mdarray` | 🔵Planned | - |
| `table` | 🔵Planned | - |
| `queue` | 🔵Planned | - |
| `buffer` | 🔵Planned | - |
| `set` | 🔵Planned | - |
| `tuple` | 🔵Planned | - |
| `pair` | 🔵Planned | - |
| `triplet` | 🔵Planned | - |
| `dict` | 🔵Planned | - |
| `string` | 🔵Planned | Single-line / Multi-line |
| `function` | 🔵Planned | - |
| `color` | 🔵Planned | Proposal stage |
| `date` | 🔵Planned | Proposal stage |
| `time` | 🔵Planned | Proposal stage |
| `duration` | 🔵Planned | Proposal stage |
| `struct` / `record` | 🔵Planned | Proposal stage |
| Type mixing | 🔵Planned | Proposal stage |

---

## Operators

| Feature | Status | Notes |
| :------ | :----- | :---- |
| `+` | 🟢Implemented | - |
| Unary `+` | 🔵Planned | - |
| `-` | 🟢Implemented | - |
| Unary `-` | 🟢Implemented | - |
| `*` | 🟢Implemented | - |
| `/` | 🟢Implemented | - |
| `//` | 🟢Implemented | - |
| `%` | 🟢Implemented | - |
| `**` | 🟢Implemented | - |
| `and` | 🟢Implemented | - |
| `or` | 🟢Implemented | - |
| `not` | 🟢Implemented | - |
| `in` | 🔵Planned | - |
| `not in` | 🔵Planned | - |
| `==` | 🔵Planned | - |
| `!=` | 🔵Planned | - |
| `>`, `>=`, `<=`, `<` | 🔵Planned | - |
| Arithmetic assignment | 🔵Planned | - |
| `&` | 🔵Planned | - |
| bar | 🔵Planned | - |
| `^` | 🔵Planned | - |
| `~` | 🔵Planned | - |
| `<<` | 🔵Planned | - |
| `>>` | 🔵Planned | - |
| Bitwise assignment | 🔵Planned | - |
| `x..y`, `x..=y`, `x..y:z` | 🔵Planned | - |

---

## Built-ins

| Feature | Status | Notes |
| :------ | :----- | :---- |
| `print` | 🟡Partial | Just positional args |
| `input`, `range`, `call` | 🔵Planned | - |
| `Logger` | 🔵Planned | - |
| Date Time | 🔵Planned | - |
| Networking | 🔵Planned | - |
| Math | 🔵Planned | - |
| File System Utilities | 🔵Planned | - |
| RegEx | 🔵Planned | - |
| Serialization | 🔵Planned | - |

---

## Attributes

| Feature | Status | Notes |
| :------ | :----- | :---- |
| `@header` | 🔵Planned | - |
| `@anonymous_shadow` | 🔵Planned | - |
| `@abstract` | 🔵Planned | - |

---

## Functions

| Feature | Status | Notes |
| :------ | :----- | :---- |
| `func` keyword | 🔵Planned | - |
| Typed parameters / return value | 🔵Planned | - |
| `return` keyword | 🔵Planned | - |
| Overloading | 🔵Planned | - |
| Variadic params | 🔵Planned | - |
| Shadow callable | 🔵Planned | - |
| Lambda functions | 🔵Planned | - |

---

## Signals

| Feature | Status | Notes |
| :------ | :----- | :---- |
| `signal` keyword | 🔵Planned | - |

---

## Enums

| Feature | Status | Notes |
| :------ | :----- | :---- |
| `enum` keyword | 🔵Planned | - |
| Anonymous enums | 🔵Planned | - |
| Enum as type | 🔵Planned | - |

---

## Condition

| Feature | Status | Notes |
| :------ | :----- | :---- |
| `if`, `elif`, `else` | 🔵Planned | - |
| `match` | 🔵Planned | - |
| Advanced pattern matching | 🔵Planned | Idea stage |

---

## Loops

| Feature | Status | Notes |
| :------ | :----- | :---- |
| `for ... in ...: ...` | 🔵Planned | - |
| `while ...: ...` | 🔵Planned | - |
| `do: ... while ...` | 🔵Planned | - |
| `break`, `continue` | 🔵Planned | Standard and extended forms |
| Advanced loop monitoring with `as` | 🔵Planned | Idea stage |

---

## Exceptions

| Feature | Status | Notes |
| :------ | :----- | :---- |
| `try` | 🔵Planned | - |
| `except` | 🔵Planned | - |
| `else` | 🔵Planned | - |
| `ensure` | 🔵Planned | - |
| `raise` | 🔵Planned | - |
| `assert` | 🔵Planned | - |

---

## OOP

| Feature | Status | Notes |
| :------ | :----- | :---- |
| `class`, `extends` | 🔵Planned | - |
| `static var`, `static func`, inner-classes | 🔵Planned | - |
| Constructors (default, static, custom, overloading, copy) | 🔵Planned | - |
| Imports (`import ...`, `exclude ...`, `... as ...`, `from ... import ...`) | 🔵Planned | - |

---

## Other / Advanced

### Language-Level Features (Core – Under Consideration)

| Feature | Status | Notes |
| :------ | :----- | :---- |
| `with ... as ...` | 🔵Planned | - |
| Async | 🔵Planned (Library-Oriented) | If introduced, should avoid low-level concurrency primitives in core |
| Macros | 🔵Planned (Restricted) | Must not introduce heavy metaprogramming or hidden behavior |
| Reflection | 🔵Planned (Limited) | Only if it preserves explicitness and clarity |
| Introspection | 🔵Planned (Limited) | Lightweight and explicit only |

### Non-Goals (Core Language)

| Feature | Status | Notes |
| :------ | :----- | :---- |
| Multi-threading / Parallel execution (low-level) | 🚫Non-Goal (Core) | Concurrency primitives not included in core; may exist via libraries |
| Heavy Meta Programming | 🚫Non-Goal (Core) | Complex compile-time manipulation avoided |
| Full Transpiler-Oriented Design | 🚫Non-Goal | Pallad is not designed primarily as a transpiled language |
| FFI (low-level system binding) | 🚫Non-Goal (Core) | Direct low-level system access avoided in core |

### Tooling / Ecosystem (Outside Core Spec)

| Feature | Status | Notes |
| :------ | :----- | :---- |
| Package Manager | 🔵 Planned (Tooling) | - |
| Formatter / Linter | 🔵 Planned (Tooling) | - |
| Debugger | 🔵 Planned (Tooling) | - |
| REPL | 🔵 Planned (Tooling) | - |
| LSP Server | 🔵 Planned (Tooling) | - |
| Syntax Highlighter | 🔵 Planned (Tooling) | - |
