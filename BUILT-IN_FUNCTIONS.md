# BadScript Built-in Functions

### `print()`
Prints the following string to stdout. Does nothing if no message is given.

Params: `msg`: string - the message to print. 

### `println()`
Prints the following string to stdout and ends with a newline. Prints
only a newline if no message is given.

Params: `msg`: string - the message to print before a newline.

### `printerr()`
Prints the given string to stderr. Simply prints "Error"
if no error message is given.

Params: `emsg`: string - the error message to print.

### `printlnerr()`
Prints the given string and a newline to stderr. Simply 
prints "Error" and a newline if no error message is given.

Params: `emsg`: string - the error message to print.

### `noop()`
Does nothing.

### `sleep()`
Waits x number of seconds (can be decimals) before continuing with the script.

Params: `sec`: number - the number of seconds to sleep for.

### (planned) `system()`
Runs the given string as a system command.

Params: `cmd`: string - the command string to run

### `exit()`
Ends the program with a code. Defaults to 0 if not specified.

Params: `code`: int - the exit code to exit on. Defaults to 0 if unspecified.
