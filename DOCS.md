
## How to run it 
Just use
```bash
liwb <file-path> ## If you have just the binary
cargo r <file-path> ## If you have the project
```

## Introduction 
Liwb, as its name suggests, works like lisp, so everything its a list.
It's also a procedural functional language, what mean that it does not have classes,
hoisting, mutability and some bloated features of modern ones.

## Syntax
The syntax in liwb are easy,
just put everything inside parenthesis and hope it works

```liwb
(define x 10) 
(print x)
```
Good syntax errors messages are overrated.
So, in liwb, to give programmers a little bit of action, 
it has no useful syntax error messages.
For example, if you forget a closing parenthesis:

```bash
Error: "Unclosed parenthesis somewhere. Good luck trying to find it."
```

## Define variables
Variables are defined with using **define** keyword.

```liwb
(define x 10)
```
Every closure in liwb returns a value, on the define closure, the value is void, which is represented by: **()**.
So, you can define a variable with the value of a variable definition.

```liwb
(define thing (define n 10))
```
Here, n becomes 10, and thing becomes void, or ().

There's no way of mutating a variable after it's definition, however,
you can just redefine it.

```liwb
(define x 10)
(define x 5) 
```
In, liwb, everything is a valid variable name, even keywords, 
as long it is not a number, a parenthesis or a bracket

```liwb
(define blah 1)
(define --- []) 
(define 1-4___-5 2) 
(define *** 3)
(define define "Define")
```

## Printing things
Every real programmer uses print statements,
which is faster and simpler.
So, of course, liwb has a print function.

```liwb
(define x 10)
(print x)
```
output:

```bash
10
```

## Basic Mathematical operations
In liwb, the 4 main operator works just like in lisp, they get accumulate.

Examples:

```liwb
(define a (+ 1 2 3 4 5))
(define b (- 1 2 3 4 5 ))
(define c (/ 1 2 3 4 5))
(define d (* 1 2 3 4 5))
(print a b c d)
```
output 

```bash
15 -13 0.008333333333333333 120
```
And, of couse, you can mix them.

```liwb
(define a (+ 1 (- 2 ( / 3 (* 4 5)))))
(print a)
```
output:

```bash
2.85
```
Liwb also have other useful math functions:

```liwb
(define a (sin 30))
(define b (sqrt 16))
(define c (mod 10 2))
(print a b c)
```
output: 

```bash
-0.9880316240928618 4 
```

The full list are:

- sqrt ## get the square root of a given number
- sin ## get the sine of a given number 
- cos ## get the cosine of a given number
- tan ## get the tangent of a given number
- abs ## get the module of a given number
- log10 ## get the log of the number in the base 10
- floor ## round down a number
- ceil ## round up a number
- round ## round a number

## Relational operators
Liwb include 5 basic relational operators

Examples:

```liwb
(print (= 10 10) (< 5 4) (> 3 0) (<= 1 3) (>= 0 0) )
```
ouput

```bash
true false true true true
```

## Data types
As every program language, Liwb have a defined set of data types.

### Numbers
Every single number as treated as a 64 bits float  number.
So, (= 10 10.0) is true in liwb.

```liwb
(print (= 10 10.0))
```
output:

```bash
true
```

Keep in mind that negative numbers do exist in liwb, 
but you can't define them directly, because i think they are bad numbers.
So, if you need a negative number for some reason, just do the following:

```liwb
(define negative (- 0 n))
```

Where n is the module of the negative number you want.

### Strings

Strings in liwb are simple definied inside double qoutes ("), 
they are not really useful here, and don't have a purpose other than beatiful priting.

```liwb
(define hello-world "Hello, World")
(print hello-world)
```
output:

```bash
Hello, World
```

If you forget to close a double qoute, you don't get a error, instead, the string becomes a variable name:

```liwb
(define name name")
```

output:

```bash
Error: "Unknow symbol name"
```

### Vectors

Vectors in liwb, as every data type, cannot be mutate,
and they can store basically everything.
Vectors are defined with brackets, [ and ].

Examples: 

```liwb
(define numbers [1 2 3 4 5])
(print numbers)
```
output:

```bash
[ 1 2 3 4 5]
```

So, because i think vectors are the cooler data type of programming, 
and i don't like negative numbers, 
you cannot define a vector with negative numbers.

```liwb
(define bad-numbers [(- 0 1) (- 0 3) (- 0 7)])
(print bad-numbers)
```
output:

```bash
[() () ()]
```
Vectors cannot store values of variables, instead, they store the variable name.

```liwb
(define name-1 "Josh")
(define name-2 "Jay")
(define names [name-1 name-2])
(print names)
```
output:

```bash
[name-1 name-2]
```

You can also create a vector with variables that does not exists:

```liwb
(define names [name-1 name-2])
(print names)
```

output:

```bash
[name-1 name-2]
```

To access a element of the vector, use the **nth** function:

```liwb
(define days [
    Monday
    Tuesday
    Wednesday
    Thursday
    Friday
    Saturday
    Sunday
])
(print (nth days 0))
```
output:

```bash
Monday
```
Remember that vector indexing starts at 0.
