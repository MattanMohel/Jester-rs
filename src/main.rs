
mod lex;
mod core;
mod prelude;

use crate::core::env::Env;

fn main() {
    let mut env = Env::new();

    env.add_module_from_file(&String::from("testing"), &String::from("src\\scripts\\jester.jt"));
}
