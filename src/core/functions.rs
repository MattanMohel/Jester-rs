
use super::exec::eval_obj;
use super::objects::{Obj, Node};
use super::env::Env;

use crate::util::scopes::Bind;

#[derive(Clone)]
pub struct JesterFn {
    body: *mut Node,
    args: *mut Node,
}

impl JesterFn {
    pub fn new(body: &mut Node, args: &mut Node) -> JesterFn {
        JesterFn {
            body: body as *mut Node,
            args: args as *mut Node,
        }
    }

    pub fn invoke(&self, env: &mut Env, args: &mut Node) -> Obj {
        let bind = Bind::bind(args);

        let mut ptr; 
        unsafe { ptr = self.body.as_mut().unwrap().copy(); }

        while !ptr.next().is_some() {
            eval_obj(env, ptr.val());
            ptr.shift();
        }

        eval_obj(env, ptr.val())
    }
}

#[derive(Clone)]
pub struct NativeFn {
    func: fn(&mut Env, Node) -> Obj
}

impl NativeFn {
    pub fn new(native: fn(&mut Env, Node) -> Obj) -> NativeFn {
        NativeFn {
            func: native,
        }
    }

    pub fn invoke(&self, env: &mut Env, args: &mut Node) -> Obj {
        (self.func)(env, args.copy())
    }
}

pub struct BridgeFn {
    name: String
}