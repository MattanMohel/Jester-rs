use std::ops::Deref;

use crate::core::{
    env::Env, 
    objects::Obj, 
    err::JtsErr,
    operations::*, 
};

impl Env {
    pub fn io_lib(&mut self) -> JtsErr {
        self.add_symbol("print", Obj::new_bridge(|env, node| {
            for obj in node.peekable() {
                print!("{}", obj.eval(env)?);
            }      

            Ok(Obj::Nil())
        }))?;

        self.add_symbol("println", Obj::new_bridge(|env, node| {
            for obj in node {
                print!("{}", obj.eval(env)?);
            }
            println!("");

            Ok(Obj::Nil())
        }))
    }
}