use super::{
    env::Env, 
    objects::Obj,
    nodes::Node
};

#[derive(Clone)]
pub struct FnBridge {
    native: fn(&Env, &Node) -> Obj,
    name: String
}

impl FnBridge {
    #[inline]
    pub fn invoke(&self, env: &Env, args: &Node) -> Obj {
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
    pub fn invoke(&self, env: &Env, args: &Node) -> Obj {
        // apply args...
        // exec body... 
        todo!()
    }
}