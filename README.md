# Vaca

Vaca is an interpreted functional programming language using a lisp-like syntax

## Features

- No side effects
- High order functions: functions are treated as values
- TODO: Partial resolution: if you don't pass enough arguments to a function then the function returns another function as the result
- Assingments: values can be assined to names inside a scope

## Syntax

### Literals

```lisp
nil ; Nil
10 ; Integer
7.78 ; Float
true ; Bool
"My name" ; String
'k' ; char
age ; Symbol
$no ; Atom [TODO]
```

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

### Functions, Macros and Calls

```lisp
<(arg arg arg ... -> expr expr expr ...)
```

Creates a function that receives some arguments and return the evaluated expression

```lisp
[(arg arg arg ... -> expr expr expr ...) ; [TODO]
```

Creates a macro that receives some arguments and returns the evaluated expression, the advantage of macros is that its arguments evaluation is lazy

```lisp
(expr expr expr ...)
```

This executes the function defined as the first argument passing all the other as it's arguments

### Array

```lisp
[expr expr expr ...]
```

This creates an array

Accessing a specific element can be done using the function `nth` with signature `(nth index array)`

### Object

```lisp
<{ key value key value key value ... } ; [TODO]
```

This creates an object associating each key to the corresponding value

Accessing the a value can be done using the function `get` with signature `(get key object)`

### Don't care [TODO]

A don't care `_` can be used to turn a call into a function by currying a value out of order

For example:

```lisp
(^ 2) ; this creates a function <(y -> (^ 2 y))
(^ _ 2) ; this creates a function <(x -> (^ x 2))
(^ _ _) ; this is the same as <(x y -> (^ x y)) or simply (^)
```

### External Libs [TODO]

Vaca (will) support loading external libraries

```lisp
<u stl/utils> ; this loads the lib utils renaming all the symbols it exports from "name" to "vutils/name"
(u/logerr ["An error message"])
```

Some more importing syntax

```lisp
<lib/path> ; loads everything from the given lib without renaming it 
<ns lib/path> ; loads everything from the given lib but preppending it's symbols with "ns/"
<... -> s1 s2 s3 ...> ; loads just the specified symbols
<: ... -> s1 s2 s3 ...> ; loads a rust dynamic library, you MUST provide the symbols to be imported
<# ... -> s1 s2 s3 ...> ; loads a c dynamic library, you MUST provide the symbols to be imported
```

Vaca will search for the libraries in the following folders:

- the folder where "vaca" was called
- "the folder where 'vaca' was called"/libs
- ${VACA_HOME}/libs (for .vaca or .casco files)
- ${VACA_HOME}/rlibs (for rust dynamic libraries)
- ${VACA_HOME}/clibs (for c dynamic libraries)

When passing a lib path to be imported there's no need for a *.vaca, *.casco, *.so, *.dll or *.dylib extension, it will be auto detected based on the import type and the platform where the code in being ran.

No spaces are allowed in the path

## Example program

```lisp
#(name "Jorge" age 19) ; Defines two variables
(print ["Hello my name is " name " and i'm " age "years old\nWhat's your name?"]) ; Calls a print
#(name (readln)) ; Reads more input from the keyboard
(print ["Nice to meet you " name "!"]) ; Prints it back
```

## TODO

- [x] Rename .leite to .casco
- [ ] STL
    - [ ] math lib
    - [ ] array lib
    - [ ] string lib
- [ ] Error Callstack
- [ ] Garbage Collector improvements
- [ ] Partial resolution
    - [x] pass less arguments then needed
    - [ ] use _ to pass away arguments out of order
- [ ] Library loading
    - Load some .vaca or .casco files
    - Load Rust dynamic libraries (.so or .dll)
    - Load C dynamic libraries (.so or .dll)
    - Load C++ dynamic libraries (.so or .dll)
- [ ] Casco CLI tool
    - A project manager for Vaca

## Project Structure

vaca - main project
vaca-core - the essential types of vaca
vaca-runtime - the runtime that runs vaca compiled code
vaca-build - the responsible of taking source code and generating an ast
vaca-repl - all the repl environment

## Future Projects

### Casco

Casco will be a Vaca project manager letting you manage the dependencies of your vaca program

### Ubre

The next Vaca garbage collector
