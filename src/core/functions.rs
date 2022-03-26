use super::{
    env::Env, 
    objects::Obj,
    nodes::Node
};

pub type BridgeFn = fn(&Env, &Node) -> Obj;

#[derive(Clone)]
pub struct FnBridge {
    bridge: BridgeFn,
}

impl FnBridge {
    pub fn new(bridge: BridgeFn) -> Self {
        Self {
            bridge: bridge
        }
    }

    #[inline]
    pub fn invoke(&self, env: &Env, args: &Node) -> Obj {
        (self.bridge)(env, args)
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