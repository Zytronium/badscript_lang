# TODO: Add loops

### `for` loops
Runs the code in brackets repeatedly while iterating a variable until a condition is false.
Checks if the condition is true or false before each iteration, so if the condition is already
false before the first iteration, it does nothing. Variable it creates is dropped when the loop
ends (meaning it's scope is this loop only).

Planned syntax:
```
for i in 0..=25 {
    println(i)
    # "continue" will skip to next iteration without finishing this one
    # "break" will end the loop here without finishing iteration
}
```

### `while` loops
Runs the code in brackets until the condition is false. Checks the if the condition is true or false before 
each iteration, so if the condition is already false before first iteration, it does nothing.

Planned syntax:
```
var new_string = ""
while len(new_string) < 12 {
    new_string += "A"
    # "continue" will skip to next iteration without finishing this one.
    # "break" will end the loop here without finishing iteration
}
println(new_string) # AAAAAAAAAAAA
```

### `do {} while` loops
Runs the code in the brackets once without checking the condition,
then if the condition is true after first run, loops until the condition is false.
Checks if condition is true or false before each run except first, so it will always
run at least once. Condition can even be false before first run and true after first run,
causing it to loop anyway.

Planned syntax:
```
i = 1
do {
    println(i)
    i += 1
} while i < 0 # only loops if i is negative, which is always false
# prints "1"
```
