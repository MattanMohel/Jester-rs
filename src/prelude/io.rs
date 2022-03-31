use std::ops::Deref;

use crate::core::{
    env::Env, 
    objects::Obj, 
    err::JtsErr,
    operations::*, 
};

impl Env {
    pub fn io_lib(&mut self) -> JtsErr {

        self.add_symbol("print", Obj::new_bridge(|_, node| {
            for obj in node {
                print!("{}", obj);
            }

            Ok(Obj::Nil())
        }))
    }
}