# TODO: Add aliases

An alias is a custom user-defined name that when found outside of strings,
comments, or function/variable names, replaces itself with the defined snippet
of code, valid or invalid syntax. Alias translation values must be in double 
quotes, and to include double quotes in the value, you can escape them with a 
backslash like this: `"quote: \" <-- quote"` = "`quote: " <-- quote`"

Example:
```
alias ispi = "= 3.1415926535897932384626"
alias prt (msg) = "println(msg)"
alias syntaxError = "\"12 =+ \"E\" * / \"\n 1?4"

var x = 17
var y ispi # sets var y to pi
var z = "I'm a number!"

prt("x:")
prt(x)
prt("y:")
prt(y)
prt("z:")
prt(z)
prt()
prt("Z is a fake number!")
prt(syntaxError) # Translates to 'println("12 =+ "E" * / "\n 1?4)' and raises syntax error
```
