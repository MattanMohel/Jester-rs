use std::{ops::Deref, borrow::Borrow};

use crate::core::{
    env::Env, 
    objects::Obj, 
    err::JtsErr,
    functions::FnNative, 
};

impl Env {
    pub fn std_lib(&mut self) -> JtsErr {
        self.add_symbol("set", Obj::new_bridge(|env, node| {
            let res = env.eval(node.get(1)?.deref())?;
            node.get_mut(0)?.set(&res);
            Ok(res)
        }))?;

        Ok(())
    }
}