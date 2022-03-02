
mod core;
mod lex;
mod util;
mod prelude;

use crate::core::objects::Obj;
use crate::core::types::Type;

struct Test {
    pub name: String,
}

impl Test {
    pub fn new(name: &str) -> Test {
        Test {
            name: name.to_string()
        }
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        println!("dropped {}!", self.name);
    }
}

fn main() {
    let mut obj = Test::new("test");
    Test::new("stat");

    let p1 = &mut obj as *mut Test;
    let p2 = &mut obj as *mut Test;

    unsafe {
        let mut obj1 = p1.as_mut().unwrap();
        let mut obj2 = p2.as_mut().unwrap();
    }

    println!("still workin'!")
}
