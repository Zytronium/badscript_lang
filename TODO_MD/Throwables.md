# TODO: Add throwable errors/exceptions + Try/Catch blocks

Add support for throwing and catching errors/exceptions. This allows scripts
to handle runtime errors gracefully, implement custom error handling logic,
and throw & catch custom exceptions.

## Syntax Example
```
var error_encountered = False

try {
    if 2 + 2 != 4 { # Probably only possible if a cosmic ray flips a bit
        throw Error("The math aint mathin'")
    }
    println("2 + 2 = 4 (as expected).")
} catch Error as err {
    error_encountered = True
    printlnerr("An impossible error has somehow occurred: ${err.msg}")
    println("Continuing anyway...")
}

if error_encountered {
    println("Error handled gracefuly.")
} else {
    println("No errors encountered!")
}

```
