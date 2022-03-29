use super::{
    env::Env, 
    objects::Obj,
    nodes::{Node, NodeIter}
};

pub type Bridge = fn(&Env, &mut NodeIter) -> Obj;

#[derive(Clone)]
pub struct FnBridge {
    pub bridge: Bridge,
}

impl FnBridge {
    #[inline]
    pub fn invoke(&self, env: &Env, args: &mut NodeIter) -> Obj {
        (self.bridge)(env, args)
    }
}

#[derive(Clone)]
pub struct FnNative {
    pub body:   Node,
    //pub params: Node,
}

impl FnNative {
    #[inline]
    pub fn invoke(&self, env: &Env, args: &mut NodeIter) -> Obj {
        // apply args...
        // exec body... 
        todo!()
    }
}