use super::{
    env::Env, 
    objects::{Node, NodeIter, Obj}
};

#[derive(Clone)]
pub struct FnBridge {
    native: fn(&Env, &NodeIter) -> Obj,
    name: String
}

impl FnBridge {
    #[inline]
    pub fn invoke(&self, env: &Env, args: &NodeIter) -> Obj {
        (self.native)(env, args)
    }
}

#[derive(Clone)]
pub struct FnNative {
    body:   Node,
    params: Node,
    name: String
}

impl FnNative {
    #[inline]
    pub fn invoke(&self, env: &Env, args: &NodeIter) -> Obj {
        // apply args...
        // exec body... 
        todo!()
    }
}