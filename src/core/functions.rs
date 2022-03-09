
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

        let mut ptr = unsafe { &mut (*self.body) };

        while !ptr.next.is_null() {
            eval_obj(env, ptr.val());
            ptr.shift();
        }

        eval_obj(env, ptr.val())
    }
}

#[derive(Clone)]
pub struct NativeFn {
    func: fn(&mut Env, &mut Node) -> Obj
}

impl NativeFn {
    pub fn new(native: fn(&mut Env, &mut Node) -> Obj) -> NativeFn {
        NativeFn {
            func: native,
        }
    }

    pub fn invoke(&self, env: &mut Env, args: &mut Node) -> Obj {
        (self.func)(env, args)
    }
}

pub struct BridgeFn {
    name: String
}