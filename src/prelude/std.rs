use std::ops::Deref;

use crate::core::{
    env::Env, 
    objects::Obj, 
    err::JtsErr,
    functions::FnNative, 
};

impl Env {
    pub fn std_lib(&mut self) -> JtsErr {

        // (defun main ...)
        self.add_symbol("defun", Obj::new_bridge(|_, node| {
            let mut fun = node.get_mut(0);

            fun.set_to(FnNative {
                body: node.get(1).is_node()?.clone()
            });

            Ok(fun.clone())
        }))
    }
}