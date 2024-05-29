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
The define closure always returns void, which is represented by: **()**.
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

## Printing things
Every real programmer uses print statements,
which is faster and simpler.
So, of course, liwb has a print function.

```liwb
(define x 10)
(print x)
```
output

```bash
10
```
