use std::ops::Deref;

use super::{
    objects::Obj, 
    nodes::Node,
    env::Env, 

    functions::Callable,
    
    err::{
        JtsErr,
        JtsErrType::*,
    }, 
};

impl Env {
    pub fn eval(&self, obj: &Obj) -> JtsErr<Obj> {
        match obj {
            Obj::Node(node) if !node.is_empty() => {
                match node.get(0)?.deref() {
                    Obj::FnBridge(_) | 
                    Obj::FnNative(_) | 
                    Obj::FnStatic(_) if !node.args.is_empty() => self.exec(node),
    
                    _ => { 
                        let args = node.into_iter().try_collect(|obj| { self.eval(obj.deref()) })?;
                        Ok(Obj::new_const(args))
                    }
                }          
            }

            _ => Ok(obj.clone())
        }
    }

    #[inline]
    fn exec(&self, node: &Node) -> JtsErr<Obj> {
        match *node.get(0)? {
            Obj::FnBridge(ref bridge_fn) => bridge_fn.invoke(self, &mut node.into_iter_from(1)),
            Obj::FnNative(ref native_fn) => native_fn.invoke(self, &mut node.into_iter_from(1)),
            Obj::FnStatic(ref static_fn) => static_fn.invoke(self, &mut node.into_iter_from(1)),
            _ => Err(NonCallable)
        } 
    }
}