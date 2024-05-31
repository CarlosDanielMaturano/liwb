## Summary 
- [How to Run It](#how-to-run-it)
- [Introduction](#introduction)
- [Syntax](#syntax)
- [Comments](#commens)
- [Variables](#variables)
- [Printing Things](#printing-things)
- [Basic Mathematical Operations](#basic-mathematical-operations)
- [Relational Operators](#relational-operators)
- [Data Types](#data-types)
  - [Numbers](#numbers)
  - [Strings](#strings)
  - [Vectors](#vectors)
- [Flow Control Operators](#flow-control-operators)
- [Functions](#functions)


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
The syntax in Liwb is straightforward: just put everything inside parentheses and hope it works.

```liwb
(define x 10) 
(print x)

```liwb
(define x 10) 
(print x)
```
Good syntax errors messages are overrated.
So, in liwb, to give programmers a little bit of action, 
it has no useful syntax error messages.
For example, if you forget a closing parentheses:

```bash
Error: "Unclosed parentheses somewhere. Good luck trying to find it."
```

## Comments
Liwb follows the philosophy that your code should be self-explanatory, so comments do not exist here.

## Variables
Variables are defined using the **define** keyword.

```liwb
(define x 10)
```
Every closure in liwb returns a value, on the define closure, the value is void, which is represented by: **()**.
So, you can define a variable with the value of a variable definition.

```liwb
(define thing (define n 10))
```
Here, n becomes 10, and thing becomes void, represented by ().

There's no way of mutating a variable after it's definition; however,
you can just redefine it.

```liwb
(define x 10)
(define x 5) 
```
In, liwb, everything is a valid variable name, even keywords, 
as long it is not a number, a parentheses or a bracket

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

## Basic Mathematical Operations
In Liwb, the four main operators work just like in Lisp; they accumulate their arguments.

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

The full list is:

- sqrt: Get the square root of a given number
- sin: Get the sine of a given number
- cos: Get the cosine of a given number
- tan: Get the tangent of a given number
- abs: Get the absolute value of a given number
- log10: Get the logarithm of the number in base 10
- floor: Round down a number
- ceil: Round up a number
- round: Round a number

## Relational operators
Liwb include 4 basic relational operators

- = -> equals operator
- > -> bigger than operator
- < -> less-than operator
- >= -> bigger-or-equal-than operator
- <= -> less-or-equal-than operator
- != -> not-equal operator

Examples:

```liwb
(print (= 10 10) (< 5 4) (> 3 0) (<= 1 3) (>= 0 0) (!= 3 3))
```
ouput

```bash
true false true true true false
```

## Data Types
Like every programming language, Liwb has a defined set of data types.

### Numbers
Every single number is treated as a 64-bit float number in Liwb. So, (= 10 10.0) is true.

```liwb
(print (= 10 10.0))
```
output:

```bash
true
```

Keep in mind that negative numbers do exist in Liwb,
but you can't define them directly,
because i think they are bad numbers.
So, if you need a negative number for some reason, just do the following:

```liwb
(define negative (- 0 n))
```

Where n is the module of the negative number you want.

### Strings

### Strings

Strings in Liwb are simply defined inside double quotes ("), 
but they are not particularly useful and don't serve a purpose 
other than beautiful printing.

```liwb
(define hello-world "Hello, World")
(print hello-world)
```
output:

```bash
Hello, World
```

If you forget to close a double quote,
you won't get an error. 
Instead, the string becomes a variable name:

```liwb
(define name name")
```

output:

```bash
Error: "Unknow symbol name"
```

### Vectors

Vectors in Liwb, like every data type, cannot be mutated,
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

Vectors are lazily evaluated, 
so instead of storing the value of an operation, 
they store the operation itself, and only get evaluated when you access them.

```liwb
(define bad-numbers [(- 0 1) (- 0 3) (- 0 7)])
(print bad-numbers)
```
output:

```bash
[ 
(liwb list#[MathOperator(Subtract), Number(0.0), Number(1.0)])
(liwb list#[MathOperator(Subtract), Number(0.0), Number(3.0)])
(liwb list#[MathOperator(Subtract), Number(0.0), Number(7.0)])  
]
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
#### Vector functions
In liwb, there are 4 main  functions for working with vectors.

- nth -> it's used for acessing elements of a vector.

```liwb
(define days [
    "Monday"
    "Tuesday"
    "Wednesday"
    "Thursday"
    "Friday"
    "Saturday"
    "Sunday"
])
(print (nth days 0))
```
output:

```bash
"Monday"
```

Remember that vector indexing starts at 0.

Inspired by Javascript, out of bound indexing on a vector returns void, 
not a anoying error.

```liwb
(define vector [])
(print (nth vector 10))
```
output:

```bash
()
```

- join -> for creating a new vector with a new element inside it.

Example:

```liwb
(define numbers [1 2 3 4])
(print (join numbers 5))
```


output:

```bash
[1 2 3 4 5] 
```

range -> for creating a vector of numbers from a given range

Example:

```liwb
(define numbers (range 1 10))
(print numbers)
```
output:

```bash
[1 2 3 4 5 6 7 8 9 10] 
```

- map -> for mapping a function to a vector, 
returns a new vector with the result of the function call on each element.

Example:

```liwb
(fn is-even [number]
    (= 0 (mod number 2)))
(define numbers (range 1 6))
(print (map is-even numbers))
```

output:


```bash
[false true false true false true] 
```

- filter -> a function that selects elements from a vector based on a provided function.

Example:

```liwb
(fn is-even [x] 
    (= 0 (mod x 2))))

(define numbers (range 1 10))
(define even-numbers (filter is-even numbers))
(print even-numbers)
```

output:

```bash
[2 4 6 8 10] 
```

## Flow Control Operators

For keeping things simple, in Liwb, the only flow control operator is the *if*.
The *if* closure consists of a (statement), a (left side), and a (right side).
It returns the left side if the statement is true, otherwise, the right side. 
The sides on a if closure are lazily evaluated.

Examples:

```liwb
(define a 10)
(print (if (>= a 10)
    "a is equal or bigger than 10"
    "a is less than 10"))

(define b 5)
(print (if (>= b 10)
    "b is equal or bigger than 10"
    "b is less than 10"))
```

output: 

```bash
"a is equal or bigger than 10" "b is less than 10"
```

## Functions
In liwb, for creating a function, you start with the *fn* keyword, 
and, after that you need 3 things: the function name,
the arguments, and the function body.

The function naming rules are the same for variables.

The paramaters are defined inside a vector.

The body is a single list.

Example:

```liwb
(fn is-even [x] 
    (= 0 (mod x 2))))
```

is-even is the function name, 
x is a single paramater, and the next thing is the function body.

The argument for a function can be zero or more.

Example:

```liwb
(fn function-without-arguments [] 
    ())
(fn function-with-multiple-arguments [a b c d e f g and-so-on ] 
    ())
```

To call funciton, just put it as the first thing inside a list, after that, put the arguments.

Example:

```liwb
(fn print-n [n] 
    (print n))
(print-n 10)
```
output

```bash
10
```

You can use recursion in functions:

Example:

```liwb
(fn fib [n]
    (if (<= n 2)
        (if (= n 1) 
            0
            1)
        (+ (fib (- n 1)) (fib (- n 2)))))

(print (map fib (range 1 13)))
```

output

```bash
[0 1 1 2 3 5 8 13 21 34 55 89 144]
```

## Delete Literals
Liwb has a magic function, the *delete* funciton, and is for, guess what, deleting literals

Lets say you dont want a program to use a bad number, so, simply delete it 

```liwb
(delete 69)
(define  a 69)
```

output:

```bash
Error: "Trying to evaluate a deleted literal: Number(69.0)"
```
