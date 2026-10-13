# BadScript
The bad scripting language that's always BS (BadScript).

BadScript is my first (technically second) attempt at building an interpreted
scripting language. It's called BadScript because it's probably not going to
be very good, and that's okay, because it's the experience and skills gained
that matter most here. This is not designed to be useful the way C, Kotlin,
or Python are designed to be useful. This is just an experiment.

**Intended Syntax**:
```
# This is a comment

func greet(name) {
    println("Hello ${name}!")
}

# Count to 25
for i in 1..=25 {
    if i == 3 {
        greet("world")
        continue
    }
    println(i)
}

try {
    if 2 + 2 != 4 {
        throw Error("The math isn't mathing!")
    } else if 2 + 2 = 5 { # unreachable
        println("I don't trust if statements")
        exit(1)
    } else {
        println("2 + 2 = 4.")
    }
} catch Error as e {
    printlnerr("An impossible error has occurred.")
    println("Continuing anyway.")
}

var x = 14

do {
    println("And now a message from our sponsor, if we had one!")
    x += 1
    println("x now equals ${x}")
} while x < 10
# ^ Only runs once ^

print(".")
sleep(0.5)
print(".")
sleep(0.5)
println(".")
sleep(0.5)

println()

println("Program has ended, goodbye! :D")
printlnerr(";)") # prints wink face in red

```

Read the files in `TODO_MD` for more information about the vision of the language. 
This directory does not cover everything - just everything that's been thought of
so far. 
