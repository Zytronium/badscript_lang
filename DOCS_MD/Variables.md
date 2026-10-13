# Variables

Variables can be declared and assigned with the following syntax:

```
var n = 12
var THIS_IS_A_CONSTANT_VALUE = True
var reallyAmazingVariable_NameIGuess_maybebut_IsThis_too_LongOh_yeahIAlmost_forgot_ThisVariable_IsThe_counterFor_LoopIteration_once_iterationExists = "0".parse_int()
var name = "John"
var nothing = Null
```

Variables can be updated with the following syntax:
```
n = 19
THIS_IS_A_CONSTANT_VALUE = False # Possible but not best practice - all caps name indicates a constant value. Will generate a warning before running script.
i += 1
name="Johnathon"
name += " Smith"
nothing = "something" # Changes type of var from Null to String
```

> Note: incrementing, decrementing, and concatenating strings with `+=` and `-=` is not implemented yet. 

This will fail: 
```
name = "Johnathon Smith"
name += 2 # Type error: cannot add number to string (or maybe we can make it work by auto-converting any type to string?)

age = 22
age += "eleven" # Type error: cannot add string to number
```

## Printing Variables

Variable values can be printed to the terminal using any of the four built-in print functions:

```
println(name)
# prints "Johnathon Smith" without the quotes
printerr("Error: input cannot be an SQL query to drop a table. Invalid input \"")
printerr(input)
printlnerr("\"")
# prints in red, 'Error: input cannot be an SQL query to drop a table. Invalid input "; DROP TABLE Responses;"'
```

> Note: String interpolation (formatting strings to include variables mid-string, such as `"Hello ${name}"`) is
planned as a separate future feature.
