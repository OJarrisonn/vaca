# The Vaca Programming Language

> Vaca is still alpha.
> Don't use it for serious projects.
> Lot of ground breaking changes may happen.
> No backwards compatibility guaranteed.

Vaca is an interpreted functional programming language that uses a LISPy syntax and is built on top of [Rust](https://rust-lang.org).

"Vaca" comes from the portuguese word for "cow". It was born as a sandboxing for a future bigger project, but now Vaca has its own ambitions. Currently targeting a tree-walker interpreter, Vaca aims to be dynamic and expansible by nature.

## Usage

The vaca executable has four subcommands:

- `help`: shows a help page
- `repl`: the default mode, starts a repl environment
- `run <file>`: runs a `*.vaca` or `*.casco` file as a program
- `build <source> [output]`: reads an input `*.vaca` file, builds and saves it to a `*.casco` file (output name is optional)

## Vaca 101

I'm assuming that you have some knowledge about programming. Let's take a look at how to use vaca

### Values

Vaca is dynamicaly typed, and those are the available values:

- Integers: `-100`, `78`, `0`
- Floats: `3.1415`, `-.5` (same as `-0.5`), `7.0`
- Strings: `"Alphabet"`, `"Hello World!"`
- Chars: `'g'`, `'r'`, `'e'`, `'a'`, `'t'`
- Booleans: `true`, `false`
- Arrays: `[]`, `[1 2 3 4]`, `["yellow" true 87 9.98]`
- Functions
- Macros
- Nil: `nil`
- NotANumber (you can't instantiate NaN, but it may be returned by math functions)

### Assignments

To assign values to symbols (commonly known as "variables") we use `#( )` syntax. They are accessible and can be overwritten in inner scopes, but once the scope is gone, the previous value is restored. They can be also overwritten in the current scope.

To create assignments, just list the symbols with their respective values.

```lisp
#(name "Jorge Harrisonn"
  age 19
  human true)
```

Try ovewritting

```lisp
#(a 10 b 25)
{
    #(a 71) ; a = 71, b = 25
}
; a = 10, b = 25
```

Symbols can be `kebab-case` with numbers and uppercases (but it must start in a letter) and my finish with `!`, `?` and/or `'` (in that order). They can also be any non reserved sequence of: `!`, `@`, `#`, `$`, `%`, `&`, `*`, `-`, `+`, `=`, ```, `~`, `^`, `:`, `>`, `<`, `,`, `|`, `\\`, `/`, `?`

### Calls

To call functions and macros use `( )` syntax, pass a callable value as the first element, and it's arguments as the remainders. If you pass more arguments than needed by the function and error is thrown, and if you pass less arguments, a new function is created curring the missing arguments.

```lisp
#(name "Harrisonn"
  age 19)
(println ["Hello World, I'm " name " and I'm " age " years old"])
(println ["This year i'll turn " (+ age 1)])

#(inc (+ 1))
(println (inc 100))
```

Check [reference](https://github.com/OJarrisonn/vaca/blob/master/REFERENCE.md) to see all the STL functions

### Code Blocks

Code blocks `{ }` are used to execute multiple expressions in sequence, but only the value of the last expression is returned. Also, a code block is a scope, so assignments inside a code block are dropped and the end of the block.

```lisp
#(test 1234)
#(res {
    #(test "Apple"
      value false)
      test
})
; res = "Apple"
; value no longer exists
; test is 1234 again
```

### Arrays

Arrays can be created by using `[ ]` syntax and then listing the values, don't need to be literal values, any Vaca expression is acceptable.

```lisp
#(data [1 'b' "three" true nil])
```

### Functions

To create functions, we use `<( -> )` syntax. List the function parameters before `->` and the forms to be evaluated after it, the value of the last form is the return value of the function. There's no early return.

```
#(cube <(x -> (^ x 3)))
(println (cube 3)) ; 27

#(hip <(b c -> (brt (+ (* b b) 
                       (* c c)) 
                    2)))
```

Pay attention that the arguments of functions are evaluated before right before the execution of the function

### Macros

The Vaca macros are symply functions whose arguments are lazy evaluated, it means, they are only evaluated when needed

```lisp
#(two-times [(x -> {x x})) ; Macro
(two-times (print "Hi")) ; HiHi
#(two-times <(x -> {x x})) ; Function
(two-times (print "Hi")) ; Hi
```

## License

MIT