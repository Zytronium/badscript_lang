# TODO: Add user-defined functions

Functions are user-defined methods that run code when called. 

Function definition syntax:

```
func self_destruct(countdown = 10) {
    var i = countdown
    
    printlnerr("WARNING: SELF DESTRUCT SEQUENCE INITIATED.")
    
    sleep(1)
    
    while i <= 0 {
        println(i)
        i -= 1
        sleep(1)
    }
    
    printlnerr("GAME OVER.")
    exit()
}

func assert_not_equal(arg1, arg2) {
    if arg1 == arg2 {
        printlnerr("arg1 is not equal to arg2")
        return False
    }
    
    return True
    # anything after here will not run
    println("this line is unreachable")
}

if assert_not_equal(2, "2") {
    println("number 2 is not equal to string \"2\".")
} else {
    printlnerr("FATAL ERROR: Number 2 and string \"2\" evaluated as equal. Initating self-destruct sequence.")
    self_destruct()
}

println("Test passed.")
sleep(3)

println("Self destructing anyway.")
sleep(1)
self_destruct(5)

```
