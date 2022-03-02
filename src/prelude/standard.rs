
use crate::core::env::{Env, new_native};
use crate::core::exec::eval_obj;

pub unsafe fn standard(env: &mut Env) {  
    env.add_symbol("+", new_native(|env, mut args| {
        let mut ret = eval_obj(env, args.val());

        while args.next().is_some() {
            ret.add(args.val());
            args.shift();
        }

        ret
    }));

    env.add_symbol("-", new_native(|env, mut args| {
        let mut ret = eval_obj(env, args.val());

        while args.next().is_some() {
            ret.sub(args.val());
            args.shift();
        }

        ret
    }));

    env.add_symbol("*", new_native(|env, mut args| {
        let mut ret = eval_obj(env, args.val());

        while args.next().is_some() {
            ret.mul(args.val());
            args.shift();
        }

        ret
    }));

    env.add_symbol("/", new_native(|env, mut args| {
        let mut ret = eval_obj(env, args.val());

        while args.next().is_some() {
            ret.div(args.val());
            args.shift();
        }

        ret
    }));
}