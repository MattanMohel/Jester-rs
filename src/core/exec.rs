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

impl Env {
    pub fn eval(&self, obj: &Obj) -> JtsErr<Obj> {
        match obj {
            Obj::Node(node) if !node.is_empty() => {
                match node.get(0)?.deref() {
                    Obj::FnBridge(_) | 
                    Obj::FnNative(_) | 
                    Obj::FnRust() if !node.args.is_empty() => self.exec(node),
    
                    _ => Ok(obj.clone())
                }          
            }      

            _ => Ok(obj.clone())
        }
    }

    fn exec(&self, node: &Node) -> JtsErr<Obj> {
        match *node.get(0)? {
            Obj::FnBridge(ref bridge) => bridge.invoke(self, node.into_iter().shift()),
            Obj::FnNative(ref native) => native.invoke(self, node.into_iter().shift()),
            Obj::FnRust() => unreachable!(),

            _ => Err(NonCallable)
        } 
    }
}