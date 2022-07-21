use std::ops::Deref;

use crate::core::{
    env::Env, 
    objects::Obj, 
    err::JtsErr,
};

impl Env {
    pub fn io_lib(&mut self) -> JtsErr {
        // (dsp body)
        // prints all elements of body with a new-line
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

        self.add_symbol("dsp-t", Obj::new_bridge(|env, node| {
            match env.eval(node.get(0)?.deref())? {
                Obj::U32(_) => println!("u32"),
                Obj::U64(_) => println!("u64"),
                Obj::I32(_) => println!("i32"),
                Obj::I64(_) => println!("i64"),
                Obj::F32(_) => println!("f32"),
                Obj::F64(_) => println!("f64"),
                Obj::Bool(_) => println!("bool"),
                Obj::Str(_) => println!("str"),
                Obj::FnStatic(_) => println!("static_function"),
                Obj::FnNative(_) => println!("native_function"),
                Obj::FnBridge(_) => println!("bridge_function"),
                Obj::Node(_) => println!("node"),
                Obj::Lazy(_) => println!("quote"),
                Obj::Nil() => println!("nil"),
            }

            Ok(Obj::Nil())
        }))?;

        Ok(())
    }
}