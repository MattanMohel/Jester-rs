use std::ops::Deref;

use super::{
    objects::Obj, 
    nodes::Node,
    env::Env, 
};

impl Obj {
    pub fn eval(&self, env: &Env) -> Obj {
        match self {
            Obj::Node(node) if !node.is_empty() => {
                println!("executing a list! {}", self);
                match node.get(0).deref() {
                    Obj::FnBridge(_) | 
                    Obj::FnNative(_) | 
                    Obj::FnRust() if !node.args.is_empty() => Obj::exec(node, env),
    
                    _ => Obj::Nil()
                }          
            }      

            _ => self.clone()
        }
    }

    fn exec(node: &Node, env: &Env) -> Obj {
        match *node.get(0) {
            Obj::FnBridge(ref bridge) => bridge.invoke(env, node.into_iter().shift()),
            Obj::FnNative(ref native) => native.invoke(env, node.into_iter().shift()),
            Obj::FnRust() => unreachable!(),

            _ => Obj::Nil()
        } 
    }
}