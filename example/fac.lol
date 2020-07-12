(def fac (x) 
	(ret (if (not (eq x 0))
		(* x (fac (- x 1)))
		1)))
