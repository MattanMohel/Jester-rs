use std::ops::Deref;

use super::{
    env::Env, 
    err::JtsErr,
    objects::Obj,

    nodes::{
        Node, 
        NodeIter
    },  
};

pub type Bridge = fn(&Env, &mut NodeIter) -> JtsErr<Obj>;

#[derive(Clone)]
pub struct FnBridge {
    pub bridge: Bridge,
}

impl FnBridge {
    #[inline]
    pub fn invoke(&self, env: &Env, args: &mut NodeIter) -> JtsErr<Obj> {
        (self.bridge)(env, args)
    }
}

#[derive(Clone, Default)]
pub struct FnNative {
    pub body: Node,
    pub params: Node,
}

impl FnNative {
    #[inline]
    pub fn invoke(&self, env: &Env, args: &mut NodeIter) -> JtsErr<Obj> {
        self.params.into_iter().scope(args, || {
            self.body.into_iter()
                .progn(|obj| { env.eval(obj.deref()) })
        })
    }
}