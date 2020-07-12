(def fib (n)
    (ret (if (or (eq n 0) (eq n 1))
        n
        (+ (fib (- n 1)) (fib (- n 2))))))

(def main ()
    (print fib 3)
    (print fib 5)
    (print fib 8))
