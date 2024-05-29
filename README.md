# Liwb
## Like lisp, but with *w* and *b*

![Static Badge](https://img.shields.io/badge/cargo-1.77.2%20-blue)
![Static Badge](https://img.shields.io/badge/LICENSE-MIT-green)

## What's the projects about :book:
Liwb is a simple (and stupid) lisp like interpreted programming language.
It does not have a real purpose, i just made it for fun.
Perhaps, it is capable of doing some interesting stuff.

## Examples 

### *First 13 Fibbonacci sequence*
```liwb
(fn fib [n]
    (if (<= n 2)
        (if (= n 1) 
            0
            1)
        (+ (fib (- n 1)) (fib (- n 2)))))

(print (map fib (range 1 13)))
```
output:
```bash
    [ 0 1 1 2 3 5 8 13 21 34 55 89 144  ]
```

### *Fizzbuz*
```liwb
(fn fizz-buzz [number]
    (if (= 0 (mod number 3))
        (if (= 0 (mod number 5))
                "FizzBuzz"
                "Fizz")
        (if (= 0 (mod number 5))
            "Buzz"
            number)))

(define numbers (range 1 20))
(print (map fizz-buzz numbers))
```
output:
```bash
    [ 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz 16 17 Fizz 19 Buzz  ]
```

## How to use the language and how it works :scroll:
See [Docs](./DOCS.md)

## License :balance_scale:
The Program is under the [MIT](./LICENSE) license
