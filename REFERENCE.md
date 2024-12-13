# Quick Reference to Vaca Standard Library Functions and Macros

- `+ a b`: sums `a` and `b`
- `- a b`: subtracts `b` from `a`
- `* a b`: multiplies `a` by `b`
- `/ a b`: divides `a` by `b`
- `// a b`: integer division of `a` and `b`
- `^ a b`: `a` to the power of `b`
- `> a b`: `a` greater than `b`
- `< a b`: `a` less than `b`
- `>= a b`: `a` greater or equals to `b`
- `<= a b`: `a` less or equals to `b`
- `max a b`: maximum of `a` and `b`
- `min a b`: minimum of `a` and `b`
- `mod a b`: integer division remainder of `a` divided by `b`
- `brt a b`: `b`-th root of `a`
- `== a b`: `a` equals to `b`
- `!= a b`: `a` not equals to `b`
- `& a b`: logic and of `a` and `b`
- `| a b`: logic or of `a` and `b`
- `readln`: reads a line from the terminal
- `format elems`: takes a value and turns it into a string, if an array is passed, apply to each element and concatenate the results
- `print elems`: similar to format, but prints the resulting string instead of returning it
- `println elems`: similar to print, but prints a linefeed at the end
- `parse-float text`: takes a string `text` and turns it into a float if possible (may crash)
- `parse-int text`: takes a string `text` and turns it into an int if possible (may crash)
- `concat init end`: concatenates the two vectors putting `end` at the end of `init`
- `append elem array`: returns a new array by putting `elem` at the start of `array`
- `prepend elem array`: returns a new array by putting `elem` at the end of `array`
- `nth n array`: returns the `n`-th element of `array` (the first element is the 0-th)
- `map f array`: takes a function `f` and `array` and return a new array where each element correspond the an element of the source array with the `f` applied
- `reduce f init array`: takes a function `f`, an initial value `init` and `array`, execute the function with `init` and the first element, then the result with the second, and so on until the end, returns the final result
- `scan f init array`: similar to reduce but returns an array of each application
- `assert ...`: (macro) takes an infinite amount of values and crashes the program if some value is falsy
- `if cond truth fake`: (macro) takes three forms, if `cond` evaluates to a truthy value, `truth` is evaluated and its result is returned, otherwise we evaluate `fake` and return its result
- `|>`: (macro) takes an infinite amount of forms, evaluate the first, pass it as the argument of the next form, and so on until the last form, returns the result of the last evaluation
- `pi`: 3.1415926 pi constant

## Macros

### `defmacro`

```clojure
(defmacro defmacro (symbol stl.macro/Symbol
                    params stl.macro/List)
          :options (options stl.macro/List
                    flags stl.macro/List
                    ... stl.macro/List
                    doc stl.macro/StringLiteral
                    as stl.macro/Form)
          :flags (private)
          :doc "Defines a macro")
```

### `def`

```clojure
(defmacro def (symbol stl.macro/Symbol 
               type stl.macro/Type) 
          :options (using stl.macro/List
                    doc stl.macro/StringLiteral 
                    as stl.macro/Form)
          :flags (private)
          :doc "Defines a value")
```

### `defn`

```clojure
(defmacro defn (symbol stl.macro/Symbol
                params stl.macro/List
                type stl.macro/Form)
          :options (using stl.macro/List
                    doc stl.macro/StringLiteral)
          :flags (private)
          :... (as stl.macro/Form)
          :doc "Defines a function")
```

### `defmod`

```clojure
(defmacro defmod (symbol stl.macro/Symbol)
          :options (using stl.macro/List
                    doc stl.macro/StringLiteral)
          :flags (private)
          :... (as stl.macro/Form)
          :doc "Defines a module")
```

### `defapp`

```clojure
(defmacro defapp (symbol stl.macro/Symbol)
          :options (using stl.macro/List
                    doc stl.macro/StringLiteral
                    main stl.macro/List)
          :flags (private)
          :... (as stl.macro/Form)
          :doc "Defines an application")
```

### `deftype`

```clojure
(defmacro deftype (symbol stl.macro/Symbol
                   type stl.macro/Form)
          :options (using stl.macro/List
                    doc stl.macro/StringLiteral)
          :flags (private)
          :doc "Defines a type"
          :as {:kind :type
               :symbol symbol
               :value (stl.macro/Use type using)
               :doc doc
               :vis (if private :private :public)})
```
