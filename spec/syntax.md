# Pallad Syntax Guide

This document shows basic syntax for Pallad language. This is a style guide too.

## Comments

Pallad supports single-line comments with `#` character, inline comments can be used in same way:

```pallad
# Single line comment

var name = "Ali" # Inline comment
```

You can use separate string literals as comment:

```pallad
"String literals are comments too."

"""
So you can use multi-line strings
as multi-line comment.
"""
```

Supports BBCode for comment formatting:

```pallad
# Use this with [func reset] function.
# Multi single-line comments will be merged, use [code][br][/code] tag to split line.
var time = 0

"""
String based comments will be used as docstring when they are above expression [b]without any space[/b].
"""
const NAME = "pallad"
```
Result:

> `time` _variable_  
> **->** Use this with [reset()]() function. Multi single-line comments will be merged, use `[br]` tag to split line.
>
> `NAME` _constant_  
> **->** String based comments will be used as docstring when they are above expression **without any space**.

You can use some keywords in comments (not string-based comments) to highlight important information:

```pallad
# Red: FIXME, CAUTION, DEBUG;
# Yellow: NOTE, REFACTOR, OPTIMIZE, DEADLINE, PRIORITY;
# Green: ASSIGNED_TO, REVIEW, LAST_UPDATE, CATEGORY;
# Will be highlighted for better comment structure, search, and management.
```

Also supports code regions:

```pallad
#region Region Name
# Manage code and have smoother navigation with code regions.
#endregion
```

## Constants

Use `const` keyword to define constant values. A constant without value will be `none`:
```pallad
const PI = 3.14
# Is valid, but will be [code]none[/code] forever:
const VALID
const GREETING = "Hello, World!"
```

## Variables
Use `var` keyword to define variables, can be static or dynamic typed:
```pallad
# Supports any value
var name = "Ali"
# Supports integer values
var _static_typed: int = 53
# Will be [code]none[/code] until assignment
var none_by_default
# Will be [code]none[/code] too
var none_array: array
# Type will be loaded from value
var auto_type := -1.9
```

Change value with `variable = value`:
```pallad
#From last code...

# Valid, name is dynamic typed
name = 12
# Invalid, [var _static_typed] is integer!
_static_typed = "-7"
# Valid
none_by_default = ["A"]
# Valid, value will be parsed before assignment
none_by_default = {"A": none_by_default}
# Valid, [var _static_typed] is integer, but [code]none[/code] can be passed to any variable and [var none_array] is [code]none[/code].
_static_typed = none_array
# Valid, 1 will be parsed to 1.0 ([var auto_type] is a float typed variable)
auto_type = 1
```

Also supports optional setter and getters:

```pallad
var activation_state: state[3] = none:
	setter(new_state: state[3]) -> void:
		if new_state == true:
			print("Activated")
		elif new_state == false:
			print("Activation Failed!")
		else:
			print("Reset activation state...")
		activation_state = new_state
	getter() -> string: # Can get parameters!
		if activation_state == true:
			return "Completed"
		elif activation_state == false:
			return "Failed"
		else:
			return "Not Started"
```

## Types
Each type is a class.

### Overview
| Type                    |  In Code   | | Type    |  In Code  |
|:------------------------|:----------:|-|:--------|:---------:|
| Variant                 | `variant`  | | Void    |  `void`   |
| Boolean                 |   `bool`   | | Integer |   `int`   |
| Byte                    |   `byte`   | | Char    |  `char`   |
| State                   |  `state`   | | Float   |  `float`  |
| Complex                 | `complex`  | | Array   |  `array`  |
| Multi-dimensional Array | `mdarray`  | | Table   |  `table`  |
| Queue                   |  `queue`   | | Buffer  | `buffer`  |
| Set                     |   `set`    | | Tuple   |  `tuple`  |
| Pair                    |   `pair`   | | Triplet | `triplet` |
| Dictionary              |   `dict`   | | String  | `string`  |
| Callable                | `function` | | Color   |  `color`  |

### Variant
Accepts any type.

### None
Represents the absence of a value.
- Default value for uninitialized variables and constants.
- Can be assigned to any variable regardless of type.
- Used in conditions as a third state (besides true and false).

### Void
Just for return type of functions without any return.

### Boolean
Uses `true` and `false` as booleans. `and`, `or`, and `not` operators are available.

### Integers
Integer number.
```pallad
var year := 2025
```

### Byte
One byte (8 bits).
```pallad
var eleven: byte = 00001011
var fifteen := byte("00001111")
```

### Char
One unicode character.
```pallad
var letter: char = 'A'
```

### N-Base state
Limited range integer value.
```pallad
var type: state[4] = 1 # 0, 1, 2, or 3
```

### Float
Floating point number.
```pallad
var temperature := 26.5
```

### Complex
Complex number with real and imaginary parts.
```pallad
# Both will be equal
var complex_number := Complex(3, 4)
var valid_complex := 3+4i
```

### Array
A container of values by index.
```pallad
var sample_array := [1, 2, 3]
var int_array: array[int] = [3, 2, 1]
```

Can be generated with `range`:
```
var r := range(3) # 0, 1, 2
var another_r := range(1, 3) # 1, 2
var steped_r := range(1, 7, 2) # 1, 3, 5
```

### Multi-dimensional array
Recurse arrays to keep data in more than one axis.
```pallad
var cube: mdarray[2, float] = [
	[
		[1.3,2,5],
		[-6,0.1,7.5],
	], [
		[5,-2.0,0.34],
		[0,0,14]
	], [
		[6,1.0,-100],
		[0.5,0.5,0.5],
	],
]
```

### Table
2-dimensional array with optional header and typed columns.
```pallad
var data: table[4, string, int, string, string] = [
	["UserID", "Age", "Username", "Password"],
	["U_54xC", 21, "Max", "M2002X"],
	["U_3z62", 23, "Joe", "LaT3X!01"],
]
```

### Queue
Special array to manage queues.
```pallad
# Add at the end
# Remove from the first
var tasks: queue[string] = Queue() # For create empty queue
tasks.enqueue("Task 1") # ["Task 1"]
tasks.enqueue("Task 2") # ["Task 1", "Task 2"]
print(tasks.dequeue()) # prints "Task 1", tasks will be: ["Task 2"]
```

### Buffer
Array of bytes.
```pallad
var file: buffer = [
    01001001, 10001101, 00000000,
    11100100, 00010011, 00000001,
]
var message := buffer("Hello World!", buffer.ENCODE_UTF_8)
```

### Set
Existence-based container.
```pallad
var s: set[int] = {1, 4, 5, 6}
```

### Tuple
Index-based container with limited size.
```pallad
var t: tuple[int, string, bool] = (42, "Hello", true)
```

### Pair & Triplet
Tuples with 2 and 3 size.
```pallad
# Use size.x or size.y to access values
# size[0] and size[1] are valid too
var size: pair[float, string] = (5.4, "MB")

# Use pos.x, pos.y, or pos.z to access values
# pos[0], pos[1], and pos[2] are valid too
var pos: triplet[int] = (3, -10, 0)
```

### Dictionary
Key-value container.
```pallad
var passwords: dict[string, string] = {"Admin": "1234", "User1": "cx?!@63"}
```

### String
Sequence of characters.
```pallad
var font: string = "Vazirmatn"
var single_letter := "V" # Will be string
```

### Callable function
A function that can be called.
```pallad
var to_int: function = func(data: string) -> int: return int(data)
```

### Color
RGBA color.
```pallad
var red := Color.RED
var blue: color = Color(0, 0, 1)
var transparent := Color(1, 1, 1, 0)
```

## Functions
Will be defined with `func` keyword, return type is optional. Overloading is supported.
```pallad
func _on_data_received(data: string) -> void:
    print(f"Data received: {data}")
```

`...` can be used to pass any count of parameters:
```pallad
func sum(...nums: array[int]) -> int:
	var sum: int = 0
	for i in nums:
		sum += i
	return sum
```

`call()` function can be used to call functions, all functions are available as callable in current scope:
```pallad
"""
After above code...
"""
call(sum, 4, 5) # Returns 9
```

### Callable (Lambda)
```pallad
var to_int: function = func(data: string) -> int: return int(data)
```

## Signals
```pallad
signal data_received(data: string)

data_received.connect(_on_data_received)

data_received.emit("Test")
```

## Enum
```pallad
# anonymous enum
enum:
    ON, # 0
    OFF, # 1
    BOTH, # 2, last "," is optional

print(OFF) # 1

@anonymous_shadow # Will be accessible without [code]Error.[/code] prefix.
enum Error:
    OK, # 0
    ERROR, # 1

print(OK) # 0
print(Error.OK) # 0

enum States:
	DAY, # 0
	NIGHT = 6,
	MIDNIGHT = 11,
	MORNING, # 12 (from last)

print(States.DAY) # 0
print(DAY) # Error, doesn't exist!

# Named enums can be used as type:
var time: States = States.DAY
```

## Condition

`if`, `elif`, and `else` are supported:
```pallad
if is_here:
    pass
elif is_valid:
    pass
else:
    pass
```

You can use `switch`, just one branch will be executed:
```pallad
enum TimeState:
	MORNING,
	DAY,
	NIGHT,
	MIDNIGHT,

switch time:
	case TimeState.DAY:
		pass
	case TimeState.NIGHT, TimeState.MORNING:
		pass
	default:
		pass
```

## Loops
`for`, `while`, and `do ... while` are available:
```pallad
for i in range(1, 10):
	if i % 2:
		continue
	if i == 9:
		break
	print(i)

while condition:
	pass

do:
	pass
while condition

for i in range(1, 10):
	for j in range(1, 10) as inner_status: # as inner_status is optional advanced loop monitoring
		if i == j:
			break(2) # breaks two loops
		if i == 9 and j == 9:
			continue(2) # skips this iteration and next one
		if i == 10:
			continue(loops=2) # skips this iteration and current iteration in outer loop
	match inner_status.status:
		case LOOP_STATUS_COMPLETE:
			print("Inner loop was passed completely.")
		case LOOP_STATUS_FULL_SKIP:
			print("Inner loop wasn't executed any time.")
		case LOOP_STATUS_HAS_SKIP:
			print(f"Inner loop was skipped for {inner_status.skip_count} times.")
		case LOOP_STATUS_BREAK:
			print(f"Inner loop was broke at {inner_status.break_iteration} iteration.")
	print(f"Inner loop executed: {inner_status.iter_executed} of {inner_status.iter_count} ({inner_status.iter_completely_executed} iterations were completed)")
else:
    print("Nothing) # when loop number is 0 (e.g. for i in []), same as LOOP_STATUS_FULL_SKIP
```

## Operators
```pallad
var x: int = 10
var y: float = 0.5
var z: int = 8

x + y # 10.5
x - y # 9.5
x * y # 5
x / z # 1.25
x // z # 1
x % z # 2
x ** z # 10^8

x == z # false
y != z # true
x > y # true
x >= y # true
x < y # false
x <= y # false

x = 11
x++ # x = 12
x-- # x = 11
x += 1 # x = 12
x -= 2 # x = 10
x *= 0.1 # x = 1
x /= 2 # Invalid, x / 2 is valid but cann't be assigned to x from type int!
y = float(x) # y = 1.0
y /= 2 # = y = 0.5

false and true # false
true or false # true
not false # true

5 & 3 # 1
5 | 3 # 7
5 ^ 3 # 6
~5 # -6
5 << 1 # 10
5 >> 1 # 2

4 in [1,4,2,6,8,2,4] # true
-10 not in [-10, 0, 10] # false
```

## Exceptions
```pallad
try:
	var x: int = 10 / 0
# When error is a RuntimeError, just this block will be executed, not next one:
except RuntimeError as error:
	print("Runtime error: ", error)
except ValueError, TypeError: # Without passing error object
	print("Value or type error.")
except Error as error:
	print("Generic error: ", error)
else:
	print(f"X: {x}") # At success
finally:
	print("Cleanup...") # Even at unexpected error

raise "Custom Error!"

assert(x > 0, "X should be positive!")
```

## Logging
Supports internal logging:
```pallad
log.add("Test")
log.error(f"Error: {e}")
log.message("Red!", Color.RED)
```

## Input
```pallad
var name: string = input("Enter your name: ")
```

## Output
```pallad
print("This is message!")
print(f"Hello, {name}!")
print(""
Line 1
Line 2
Line 3
"")
```

## Files
```pallad
try:
	# "with" provides auto-close:
	with open("data.txt", "r") as file:
		print(file.read())
		for line in file.lines:
			print(f"Line: {line}")
except Error:
	print("Error!")

with open("data.txt", "w") as file:
	file.write("Some text.")

with open("data.txt", "a") as file
	file.write("This is new line.\n")
```

## OOP
Everything in Pallad is an object, data types are classes too.

```pallad
# class keyword can be used to customize class name, otherwise file name will be used.
class Person
# "extends" for objects are optional and Person is an "Object"

# Some code here...
```

```pallad
# In same folder
class Home
extends Place

# Imports eveything in current folder
import *

var persons: Array[Person]

for i in range(2):
	var p := Person()
	persons.append(p)
```

### Static properties
`static` keyword can be used for function and variables to make them static. Static functions and variables can be used without creating instance and static variables are shared between instances. `static:` block and lines outside any function or constructor will be called once per class. `static constructor -> bool:` can be used to run code before first construction and should return `true` at successful initializations.
```pallad
class User

import Game

static var count: int
static var game: Game
var name: string
var age: int

print("First static")

# This syntax improves readability but is optional: place defenitions in class body and behavior in "static:" block
static:
	count = 0
	print("Second static")

# Static behavior will be excuted in writing order
print("Last static")

static constructor -> bool:
	if game == none:
		print("Failed to initialize constructors!")
		return false
	print("Constructors initialized")
	return true

constructor(name: string, age: int):
	self.name = name
	self.age = age
	print("Instance created")

static func get_game() -> Game:
	return game

static func has_game() -> bool:
	return game != none
```

```pallad
print("Import")
import User
import Game

print(User.has_game())
var game := Game()

User.game = game
print(User.has_game())
print("Instance")
var alex := User("Alex", 32)
var joe := User("Joe", -41)
```
Output:
```
Import
First static
Second static
Last static
false
true
Instance
Contructors initialized
Instance created
Instance created
```

| Block                | Running time                                                         | Useful for                                                                  |
|----------------------|----------------------------------------------------------------------|-----------------------------------------------------------------------------|
| Free code            | Class initialization                                                 | Simple codes, static behavior, fast setup                                   |
| `static`             | Same as free code, after upper free codes and before next free codes | Same as free code with more readability                                     |
| `static cosntructor` | Before first constructor run                                         | Time-safe initializations, when you need two step initialization with delay |
| `static func`        | At call                                                              | Accessing static variables, etc.                                            |
| `constructor`        | Instance creation                                                    | Initializing instances with different values                                |

### Import
Use `import` and `exclude` keywords to manage available modules. You can use `if` and other branching keywords to manage imports.

```pallad
# Use "" for paths, otherwise don't use!

# Use exclude before imports
exclude test_module # "./test_module.pd" will not be imported anyway.

import * # Everything in current folder
import "D:/Codes/personal_utils_v1" as PUtils # Imports personal_utils_v1.pd, ".pd" is optional

from os import process, path

# config.pd exsists in current folder.
if config.enable_debugger:
	from "../debugger/"  import * # Imports every module in "debugger" folder in upper directory, last "/" is required
else:
	import "../debugger/light_debbuger" # Just imports "light_debugger.pd" module.
```

### Inner classes
```pallad
class Utils

# Inner-class decleration will be moved to top, so this line is valid:
var counter := Counter()
counter.add()
print(counter.get())

class Counter:
	var count := 0

	func add() -> void:
		count += 1
	
	func get() -> int:
		return count
```

Should be used like this in other modules:
```pallad
import Utils

var counter := Utils.Counter()
# or:
from Utils import Counter
var another_counter := Counter()
```

### Constructors
```pallad
class User

var name: string
var age: int

constructor(): # Simple constructor
	self.name = "Unknown"
	self.age = 0

constructor(name: string): # Constructor with parameter
	self.name = name
	self.age = 0

constructor(name: string, age: int): # Overloading
	self.name = name
	self.age = age

constructor(copy: User): # Copy constructor
	self.name = copy.name
	self.age = copy.age
```

## Style Guide

### Naming Rules
| Type                          | Case               | Example                              |
|-------------------------------|--------------------|--------------------------------------|
| Constant                      | `UPPER_SNAKE_CASE` | `const DEBUG = false`                |
| Variable                      | `snake_case`       | `var count := 3`                     |
| Function & Function parameter | `snake_case`       | `func reset(force := false):`        |
| Signal & Signal parameter     | `snake_case`       | `signal data_received(data: string)` |
| Enum name                     | `PascalCase`       | `enum Time:`                         |
| Enum value                    | `UPPER_SNAKE_CASE` | `DAY,`                               |
| Class name                    | `PascalCase`       | `class User`                         |