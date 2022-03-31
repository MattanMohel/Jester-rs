use std::ops::Deref;

use super::{
    objects::Obj, 
    nodes::Node,
    env::Env, 
    err::{
        JtsErr,
        ErrType::*,
    }, 
};

impl Obj {
    pub fn eval(&self, env: &Env) -> JtsErr<Obj> {
        match self {
            Obj::Node(node) if !node.is_empty() => {
                match node.get(0).deref() {
                    Obj::FnBridge(_) | 
                    Obj::FnNative(_) | 
                    Obj::FnRust() if !node.args.is_empty() => Obj::exec(node, env),
    
                    _ => Ok(Obj::Nil())
                }          
            }      

            _ => Ok(self.clone())
        }
    }

    fn exec(node: &Node, env: &Env) -> JtsErr<Obj> {
        match *node.get(0) {
            Obj::FnBridge(ref bridge) => bridge.invoke(env, node.into_iter().shift()),
            Obj::FnNative(ref native) => native.invoke(env, node.into_iter().shift()),
            Obj::FnRust() => unreachable!(),

            _ => Err(NonCallable)
        } 
    }
}