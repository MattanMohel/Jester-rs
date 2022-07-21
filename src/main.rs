
mod lex;
mod core;
mod prelude;
mod macros;

use crate::core::objects::Obj;

use crate::core::err::JtsErr;
use crate::core::env::Env;

fn add((a, b): (i32, i32)) -> i32 {
    a + b
}

fn main() -> JtsErr {
    let mut env = Env::new()?;

    // adding static function
    env.add_symbol("add-rs", Obj::new_static(add))?;

    // adding direct source
    env.add_src(
        "(defun apply (f args)
            (let 
                (( a args ))
                
                (prepend f a)
                a))

        (defun fac (n)
            (if (= n 0)
                1
                (* n (fac (- n 1)))))

        (defun range (n)
            (set i 0)
            (set lst ())
            
            (loop (< i n)
                (append i lst)
                (set i (+ i 1)))
            lst)

        (defun factorize (n)
            (set i 2)
            (set acc ())

            (loop (> n 1)
                (if (= (% n i) 0)
                    (do
                        (set n (/ n i))
                        (append i acc)
                        (set i 2))
                    nil)
                (set i (+ i 1)))

                acc)"
    )?;

    //env.run_main()?;
    env.run_repl()?;

    Ok(())
}
