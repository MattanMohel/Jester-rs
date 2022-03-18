
mod lex;
mod core;
mod util;
mod prelude;

use crate::core::env::Env;

fn main() {
    let mut env = Env::new();

    env.new_module_from_file("module", "src\\scripts\\jester.jt");
}
