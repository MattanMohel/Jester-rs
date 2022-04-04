use std::ops::Deref;

use super::{
    env::Env, 
    err::JtsErr,
    objects::Obj,

    types::TypeId,

    nodes::{
        Node, 
        NodeIter
    },  
};

pub type Bridge = fn(&Env, &mut NodeIter) -> JtsErr<Obj>;

#[derive(Clone)]
pub struct FnBridge {
    pub func: Bridge,
}

impl FnBridge {
    #[inline]
    pub fn invoke(&self, env: &Env, args: &mut NodeIter) -> JtsErr<Obj> {
        (self.func)(env, args)
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
        self.params.into_iter().scope(env, args, || {
            self.body.into_iter()
                .progn(|obj| { env.eval(obj.deref()) })
        })
    }
}

pub trait TupleCast 
    where Self: Sized 
{
    unsafe fn cast(args: &NodeIter) -> JtsErr<Self>;
}

impl TupleCast for (i32, i32) {
    unsafe fn cast(args: &NodeIter) -> JtsErr<Self> {
        Ok((
            args.get(0)?.cast_as::<i32>()?,
            args.get(1)?.cast_as::<i32>()?
        ))
    }
}

pub struct FnRust<A, R>
    where R: TypeId
{
    func: fn(A) -> R
}

impl<A, R> FnRust<A, R> 
    where A: TupleCast, R: TypeId
{
    #[inline]
    pub fn invoke(&self, env: &Env, args: &mut NodeIter) -> JtsErr<Obj> {
        let map = unsafe { A::cast(args)? };
        let res = (self.func)(map);
        Ok(Obj::new_const(res))
    }
}