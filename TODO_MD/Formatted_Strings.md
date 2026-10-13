# TODO: Add formatted strings

Formatted strings are strings that have variable values injected at specific points of the string. 
Pretty much every modern language supports this in one way or another.

Planned usage:
```
var pi = 3.14159
var formatted_string = "The first few digits of pi are ${pi}, but pi has infinite digits."

println("This message has a formatted string embedded in a formatted string. ${formatted_string} That was a formatted string.")

```

Output:
```
This message has a formatted string embedded in a formatted string. The first few digits of pi are 3.14159, but pi has infinite digits. That was a formatted string.
```
