# lol

A lisp based on [lovm2](https://github.com/lausek/lovm2).

## Example

```
(def fib (n)
    (if (or (eq n 0) (eq n 1))
        (ret n)
        (ret (+ (fib (- n 1)) (fib (- n 2))))))
```

## Builtin Macros

```
+
-
*
/
%
eq
ne
ge
gt
le
lt
and
or
break
continue
dict
do
foreach
if
import
import-global
let
list
loop
range
ret
```
