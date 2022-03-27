
mod lex;
mod core;
mod prelude;
mod macros;

use crate::core::err::ParseErrType;
use crate::core::env::Env;

fn main() -> Result<(), ParseErrType>{
    let mut env = Env::new();

    env.add_module_from_file(&String::from("testing"), &String::from("src\\scripts\\jester.jt"))?;
    Ok(())
}
