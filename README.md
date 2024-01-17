# Vaca

Vaca is an interpreted functional programming language using a lisp-like syntax

## Features

- No side effects
- High order functions: functions are treated as values
- TODO: Partial resolution: if you don't pass enough arguments to a function then the function returns another function as the result
- Ownership: values have owners, once their owner goes out of scope the value is freed
- Assingments: values can be assined to names inside a scope

## Syntax

### Assingment

```lisp
#(name value name value ...)
```

Each name is defined (or redefined) to the given value with the given type associated

### Code Blocks

```lisp
{expr expr expr ...}
```

Each expression is evaluated and the last one is returned as the value of the block

### Functions and Calls

```lisp
<(arg arg arg ... -> expr expr expr )
```

This creates a function that receives some arguments and return the evaluated expression

```lisp
(expr expr expr ...)
```

This executes the function defined as the first argument passing all the other as it's arguments

### Array

```lisp
[expr expr expr ...]
```
This creates an array

## Example program

```lisp
#(name "Jorge" age 19) ; Defines two variables
(print ["Hello my name is " name " and i'm " age "years old\nWhat's your name?"]) ; Calls a print
#(name (read)) ; Reads more input from the keyboard
(print ["Nice to meet you " name "!"]) ; Prints it back
```