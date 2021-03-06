# calculator

![](examples/example.gif)

```
>> (10) - 2 + 4;
[0] 4
>> 2 * 0x03
[1] 6
>> $ ^ $$
[2] 1296
>> $2 % 5
[3] 1
```

## Installation
```
$ cargo install --locked --git https://github.com/Techno-coder/calculator
```

## Execution
```
$ calculator
```
Basic mode does not update on each key press and may work better for less
advanced terminals.
```
$ calculator -b/--basic
```
Evaluation mode reads from the standard input pipe and outputs results directly.
```
$ echo expression | calculator -e/--evaluation
```

## Arithmetic Operators
In order of precedence:
* `+` - Add
* `-` - Minus
* `*` - Multiply
* `/` - Divide
* `%` - Modulo
* `^` - Power

## Numerical Formats
* `0x000a` - Hexadecimal
* `0b1010` - Binary
* `0o0012` - Octal
* `1.0` - Floating
* `1e1` - Scientific

## Variables
* `$` - Last evaluation result
* `$$` - Second last evaluation result
* `$$...$` - Arbitrary position evaluation result
* `$0` - Variable with identifier `0`
* `$aa` - Variable with identifier `aa`

## Functions
```
>> function argument
```
Functions take the term immediately to the right. 
Whitespace is required after the function name.

* `abs` - Absolute value
* `sqrt` - Square root
* `cbrt` - Cube root
* `ln` - Natural logarithm
* `log2` - Binary logarithm
* `log10` - Decimal logarithm

### Trigonometry
* `sin` - Sine
* `cos` - Cosine
* `tan` - Tangent
* `asin` - Inverse sine
* `acos` - Inverse cosine
* `atan` - Inverse tangent

The trigonometric functions can take and return degrees by appending `'`:
```
>> asin' 1
[0] 90
```

## Constants
* `e` - Euler number
* `pi` - Pi (3.14)

## Coalescence
* `;` - Coalesce operator

The coalesce operator combines the previous two terms into a single node.
This eliminates the need for parentheses.

```
>> 10 - 2 + 4;
```
is equivalent to:
```
>> 10 - (2 + 4)
```

Repetitions of the coalesce operator will combine more than two terms:
```
>> 1 / 1 + 2 + 3;;
```
is equivalent to:
```
>> 1 / (1 + 2 + 3)
```
Sometimes a combined term is nested inside another:
```
>> 1 / (2 - (3 + 4))
```
This can be achieved by leaving whitespace after the first coalescence:
```
>> 1 / 2 - 3 + 4; ;
```
