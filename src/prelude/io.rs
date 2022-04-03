use std::ops::Deref;

use crate::core::{
    env::Env, 
    objects::Obj, 
    err::JtsErr,
};

impl Env {
    pub fn io_lib(&mut self) -> JtsErr {
        // (print body)
        // prints all elements of body with no new-line
        self.add_symbol("print", Obj::new_bridge(|env, node| {
            node.progn_then(
                |obj| { 
                    print!("{}", env.eval(obj.deref())?);
                    Ok(())
                },
                |obj| {
                    let res = env.eval(obj.deref())?;
                    print!("{}", res);
                    Ok(res)
                })
        }))?;

        // (println body)
        // prints all elements of body with a new-line
        self.add_symbol("println", Obj::new_bridge(|env, node| {
            node.progn_then(
                |obj| { 
                    print!("{}", env.eval(obj.deref())?);
                    Ok(())
                },
                |obj| {
                    let res = env.eval(obj.deref())?;
                    println!("{}", res);
                    Ok(res)
                })
        }))?;

        Ok(())
    }
}