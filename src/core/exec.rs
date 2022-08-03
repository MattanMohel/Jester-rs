use std::ops::Deref;

use crate::core::{env::{new_shared, Shared}, types::TypeId, nodes::NodeIter};

use super::{
    functions::Callable,
    objects::Obj, 
    nodes::Node,
    err::JtsErr,
    env::Env, 
};

impl Env {
    pub fn eval(&self, obj: &Obj) -> JtsErr<Obj> {
        match obj {
            Obj::List(node) if !node.is_empty() => {
                match node.get(0)?.deref() {
                    Obj::FnBridge(_) | 
                    Obj::FnNative(_) | 
                    Obj::FnStatic(_) |
                    Obj::FnMacro(_) => self.exec(node),
    
                    _ => { 
                        let res = node.into_iter().try_map_collect(|obj| self.eval(obj.deref()))?;
                        Ok(Obj::new_const(res))
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
            Obj::FnMacro(ref macro_fn) => macro_fn.invoke(self, &mut node.into_iter_from(1)),
            _ => unreachable!()
        } 
    }
  
    pub fn eval_shared(&self, obj: &Obj) -> Result<Obj, crate::core::err::JtsErrType> {     
    
        fn clone_shared(env: &Env, obj: &Shared<Obj>) -> JtsErr<Shared<Obj>> {   
            match &obj.borrow().deref() {
                Obj::List(node) => {
                    let res = node.into_iter().try_map_collect_shared(|e| clone_shared(env, &e) )?;
                    Ok(new_shared(res.into_obj()))
                }
                Obj::Quote(quote) => Ok(quote.clone()),
                _ => Ok(obj.clone())
            }
        }

        let obj = self.eval(obj)?;
        
        match &obj {
            Obj::List(node) => {
                let res = node.into_iter().try_map_collect_shared(|e| clone_shared(self, &e) )?;
                self.eval(&res.into_obj())
            },
         
            Obj::Quote(quote) => Ok(quote.borrow().clone()),           
            _ => Ok(obj)
        }
    }
}