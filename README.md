# calculator

```
>> (10) - 2 + 4;
[ 0] 4
>> 2 * 0x03
[ 1] 6
>> $ ^ $$
[ 2] 1296
>> $2 % 5
[ 3] 1
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

## Coalescence
* `;` - Coalesce operator

The coalesce operator combines the previous two expressions into a single node.
This eliminates the need for parenthesis.

```
>> 10 - 2 + 4;
```
is equivalent to:
```
>> 10 - (2 + 4)
```

Repetitions of the coalesce operator will combine more expressions:
```
>> 1 / 1 + 2 + 3;;
```
is equivalent to:
```
>> 1 / (1 + 2 + 3)
```
Sometimes a combined expression is nested inside another:
```
>> 1 / (2 - (3 + 4))
```
This can be achieved by leaving whitespace after the first coalescence:
```
>> 1 / 2 - 3 + 4; ;
```
