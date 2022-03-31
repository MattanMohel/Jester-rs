
mod lex;
mod core;
mod prelude;
mod macros;

use crate::core::err::JtsErr;
use crate::core::env::Env;

fn main() -> JtsErr {
    let mut env = Env::new()?;

    env.add_module_from_file(&String::from("testing"), &String::from("src\\scripts\\jester.jt"))?;
    println!("{}", env.run()?);

    Ok(())
}
