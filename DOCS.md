## How to run it 
Just use
```bash
liwb <file-path> ## If you have just the binary
cargo r <file-path> ## If you have the project
```

# Basics
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
Good syntax errors messages make you feel dumb for forgeting something so simple.
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
