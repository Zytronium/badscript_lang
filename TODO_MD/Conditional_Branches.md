# TODO: Add conditional branching (if/else)

If/else blocks are conditional logic branches that run based on the 
value of a given condition or expression. The condition must always
evaluate to a boolean (i.e. `if 13 {}` raises an error but 
`if 13 <= x {}` and `if true {}` are valid). 

Planned syntax:
```
if 12 < 18 {
    println("12 is less than 18")
}

if 2 == 2 {
    println("2 is equal to itself.")
} else {
    printlnerr("ERROR: 2 is not equal to 2, apparently.")
}

var user_input = "Pretent the user typed this"

if len(user_input) < 16 {
    println("Valid input length.")
} else if len(user_input) == 16 {
    println("Input is the maximum allowed length.")
} else {
    printlnerr("ERROR: Input cannot be longer than 16 characters.")
    exit(1)
}

println("This will print if user input was under 17 characters") # exit() ends the program and the program will not reach this line if exit() was called

```
