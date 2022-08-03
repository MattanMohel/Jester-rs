
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
    // env.add_src(
    //     "
    //     (macro for (var in list body)
    //         (let ( (i (gen-sym 0)) )
    //             '(loop (< ,i (len list))
    //                 (set var (nth ,i list))
    //                 (do body))))
        
    //     ")?;

    //env.run_main()?;

    // env.add_src(
    //     "
    //     (macro += (num incr)
    //         '(set ,num (+ ,num ,incr)))

    //     "
    // )?;

    env.add_src(
    "
        (macro apply (f args)
            (let ((cpy args))
                (prepend f cpy)
                cpy))        
    ")?;

    env.add_src(
    "
        (macro += (a b)
            '(set ,a (+ ,a ,b)))       
    ")?;

    env.add_src(
    "
        (macro for (var in lst bdy)
            (let ((i (gensym 0)))
                '(let ((,var ,nil))
                    (loop (< ,i (len ,lst))
                        (set ,var (nth ,i ,lst))
                        (do ,bdy)
                        (set ,i (+ ,i ,1))))))
                
    ")?;


    env.run_repl()?;

    Ok(())
}
