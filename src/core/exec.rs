
use super::objects::Obj;
use super::types::Type;
use super::env::Env;

pub fn eval_obj(env: &mut Env, obj: &mut Obj) -> Obj {
    match obj.var {
        Type::Node(args) => unsafe {
            if !args.is_null() && args.as_mut().unwrap().val().var.is_callable() {
                return exec_obj(env, obj)
            }
        }

        Type::Ref(quote) => unsafe {
            if env.eval {
                eval_obj(env.with_eval(true), quote.as_mut().unwrap());
                env.with_eval(false);
            }
        }
        
        _ => ()
    }

    obj.clone()
}

fn exec_obj(env: &mut Env, obj: &mut Obj) -> Obj {
    todo!()
}