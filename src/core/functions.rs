use super::{
    env::Env, 
    objects::Obj,

    nodes::{
        Node, 
        NodeIter
    }, 
    
    err::{
        JtsErr,
        ErrType::*,
    }
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

#[derive(Clone)]
pub struct FnNative {
    pub body:   Node,
    //pub params: Node,
}

impl FnNative {
    #[inline]
    pub fn invoke(&self, env: &Env, args: &mut NodeIter) -> JtsErr<Obj> {
        // apply args...
        // exec body... 
        println!("native call!");
        Err(Todo)
    }
}