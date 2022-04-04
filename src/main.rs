
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

    // reading from file
    env.add_module_from_file(&String::from("main"), &String::from("D:\\repo\\Rust\\Jester-rs\\src\\scripts\\jester.jt"))?;

    // adding static function
    env.add_symbol("add-rs", Obj::new_static(add))?;

    // adding direct source
    env.add_src(
        "(defun apply (f args)
        (let 
            ((applied args))
            
            (prepend f applied)
            applied))"
    )?;

    env.run_main()?;
    env.run_repl()?;

    Ok(())
}
