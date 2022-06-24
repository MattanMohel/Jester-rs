
(macro for (i in l ..body)  
    (set it (gen-sym))
 
    (loop (< it (len l))
        (set i (nth (eval it) l))
        (apply-each eval body)))

(for i in (1 2 3)
    (println i))


(defun f (x)
    (* x x))

(defun f (x)
    (match x
        0 => 0
        x => (* x x)))



(set num (f 2))

