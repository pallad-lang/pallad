# Pallad Features

This document lists **ALL features of Pallad programming language**, currently implemented features are checked.

> [!Note]
> Items marked with "!" are still just general ideas and have a ways to go before being finalized as a feature on this list (proposal and then implementation).

## Comments
- [x] Single-line comments with `#`
- [ ] Multi-line comments with `"""`
- [ ] BBCode converter module for comments
- [ ] Highlight rules for single-line comments:
  - Red: `FIXME`, `CAUTION`, `DEBUG`
  - Yellow: `NOTE`, `REFACTOR`, `OPTIMIZE`, `DEADLINE`, `PRIORITY`
  - Green: `ASSIGNED_TO`, `REVIEW`, `LAST_UPDATE`, `CATEGORY`
- [ ] Folding rules for `#region Region Name` and `#endregion`

## Constants
- [ ] `const` keyword
- [ ] Check for duplicates
- [ ] `none` constants

## Variables
- [x] `var` keyword
- [ ] Check for duplicates
- [ ] Optional static typing
- [ ] Auto-type with `:=`
- [ ] Change
- [ ] `setter()` and `getter()`

## Data Types
- [ ] `variant`
- [ ] `void`
- [x] `none`
- [x] `bool`
- [x] `int`
- [ ] `byte`
- [ ] `char`
- [ ] `state`
- [x] `float`
- [ ] `complex`
- [ ] `array`
- [ ] `mdarray`
- [ ] `table`
- [ ] `queue`
- [ ] `buffer`
- [ ] `set`
- [ ] `tuple`
- [ ] `pair`
- [ ] `triplet`
- [ ] `dict`
- [ ] `string`
  - [x] Single-line with `"`
  - [ ] Multi-line `"""`
- [ ] `function`
- [ ] `color`
- [ ] ! `date`
- [ ] ! `time`
- [ ] ! `duration`
- [ ] ! `struct` / `record`
- [ ] ! Type mixing

## Operators
- [x] `+`
- [x] `-`
- [x] `*`
- [x] `/`
- [x] `//`
- [x] `%`
- [x] `**`
- [x] `and` `or` `not`
- [ ] `in` `not in`
- [ ] `==` `!=`
- [ ] `>` `>=` `<` `<=`
- [ ] `++` `--`
- [ ] `+=` `-=` `*=` `/=` `//=` `%=` `**=`
- [ ] `&` `|` `^` `~` `<<` `>>`
- [ ] `&=` `|=` `^=` `~=` `<<=` `>>=`
- [ ] `x..y` `x..=y` `x..y:z`

## Built-ins
- [ ] Global
  - [x] `print`
  - [ ] `input`
  - [ ] `range`
  - [ ] `call`
  - [ ] `Logger`
    - `auto_write`
    - `info`
    - `warn`
    - `error`
    - `save`
- [ ] Date Time
- [ ] Networking
- [ ] Math
- [ ] File System Utilities
- [ ] RegEx
- [ ] Serialization

## Attributes
- [ ] `@header`
- [ ] `@anonymous_shadow`

## Functions
- [ ] `func` keyword
- [ ] Typed parameters
- [ ] `return` keyword
- [ ] Typed return value
- [ ] Overloading
- [ ] Check for duplicates
- [ ] Variadic params with `...`
- [ ] Shadow callable
- [ ] Lambda functions

## Signals
- [ ] `signal` keyword
- [ ] Check for duplicates

## Enums
- [ ] `enum` keyword
- [ ] Check for duplicates
- [ ] Anonymous enums
- [ ] Enum as type

## Condition
- [ ] `if` `elif` `else`
- [ ] `match`
- [ ] ! Advanced pattern matching

## Loops
- [ ] `for ... in ...: ...`
- [ ] `while ...: ...`
- [ ] `do: ... while ...`
- [ ] `break` `break(n)`
- [ ] `continue` `continue(n)` `continue(parent=n)`
- [ ] Advanced loop monitoring with `as` keyword

## Exceptions
- [ ] `try` `except` `except as`
- [ ] `else` `ensure`
- [ ] `raise`
- [ ] `assert`

## OOP
- [ ] `class`
- [ ] `extends`
- [ ] Set default class name from file name
- [ ] `static var` `static func`
- [ ] `static`
- [ ] Inner-classes
- [ ] Constructors
  - [ ] Default constructor
  - [ ] `static constructor`
  - [ ] Custom constructors
  - [ ] Overloading
  - [ ] Copy constructor
- [ ] Imports
  - [ ] `import ...` (`import *`)
  - [ ] `exclude`
  - [ ] `import ... as ...`
  - [ ] `from ... import ...`

## Other
- [ ] `with ... as ...`
- [ ] ! Async
- [ ] ! Macros
- [ ] ! Meta Programming
- [ ] ! Reflection
- [ ] ! Introspection
- [ ] ! Multi-threading / Parallel execution
- [ ] ! FFI
- [ ] ! Transpile
- [ ] ! Package Manager
- [ ] ! Formatter / Linter
- [ ] ! Debugger
- [ ] ! REPL
- [ ] ! LSP Server
- [ ] ! Syntax Highlighter

---

## Known Issues
This is a list of known missing points about implemented features listed above:
- Parser:
  - Changing value of a variable needs `var` keyword, otherwise raises parse error `Expected 'var', 'print', or end of line, got Ident(...)`
  - Multi-line expressions raises parse error `Expected value, variable, or '(', got Eol`.
  - Using `+` before value (e.g. `+5`) raises parse error `Expected value, variable, or '(', got Plus`.
- VM:
  - Integer operations can overflow, values wrap silently.

> [!Note]
> The items on this list have been queued for resolution.