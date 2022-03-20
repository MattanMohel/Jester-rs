use super::{env::Env, objects::{Obj, Node, NodeIter}};

pub fn eval_obj(env: &Env, obj: &Obj) -> Obj {
    match obj {

        Obj::Args(args) => {
            let node = args.iter(env);

            match &node[0] {
                Obj::FnBridge(_) | Obj::FnNative(_) | Obj::FnRust() => exec_obj(env, &node),
                _ => obj.clone()
            }
        }
        _ => {
            obj.clone()
        }
    }
}

fn exec_obj(env: &Env, node: &NodeIter) -> Obj {
    match &node[0] {
        Obj::FnNative(func) => func.invoke(env, &node),
        Obj::FnBridge(func) => func.invoke(env, &node),
        _ => panic!("tried executing non-function object")
    }
}