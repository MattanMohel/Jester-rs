
mod core;
mod lex;
mod util;
use crate::core::modules::Module;

fn main() {
    
    let mut module = Module::new(&"src\\scripts".to_string(), &"jester.jt".to_string());
    module.debug();
}
