
mod lex;
mod core;
mod util;
mod prelude;

use crate::core::env::Env;
use crate::core::types::Type;
use crate::core::objects::Obj;
use crate::core::modules::Module;

fn main() {
    let mut env = Env::new();

    let mut module = Module::new(&mut env, &"src\\scripts".to_string(), &"jester.jt".to_string());

    module.debug();
}
