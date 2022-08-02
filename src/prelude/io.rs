use std::ops::Deref;

use crate::core::{
    env::Env, 
    objects::Obj, 
    err::JtsErr,
};

impl Env {
    pub fn io_lib(&mut self) -> JtsErr {
        //@decl
        // macro dsp
        //@params
        // ('body...)
        //@return
        // the evluation of body
        //@doc
        // prints out all values of 'body' followed by a new-line
        self.add_symbol("dsp", Obj::new_bridge(|env, node| {
            node.progn_then(
                |obj| { 
                    print!("{}", env.eval(obj.deref())?.to_string(env));
                    Ok(())
                },
                |obj| {
                    let res = env.eval(obj.deref())?;
                    println!("{}", res.to_string(env));
                    Ok(res)
                })
        }))?;

        Ok(())
    }
}