use std::ops::Deref;

use crate::core::{
    env::Env, 
    err::JtsErr,
    objects::Obj, 
};

impl Env {
    pub fn std_lib(&mut self) -> JtsErr {
        // constant true value
        self.add_symbol("T", Obj::new_const(true))?;
        // constant false value
        self.add_symbol("F", Obj::new_const(false))?;
        // constant pi value
        self.add_symbol("pi", Obj::new_const(3.1415926535))?;

        // (set target value)
        // sets target to a copy of value
        self.add_symbol("set", Obj::new_bridge(|env, node| {
            let res = env.eval(node.get(1)?.deref())?;
            node.get_mut(0)?.set(&res);
            Ok(res)
        }))?;

        // (= value cmpr)
        // returns boolean value of expreesion 'value == cmpr'
        self.add_symbol("=", Obj::new_bridge(|env, node| {
            let res = Obj::eq(&env.eval(node.get(0)?.deref())?, &env.eval(node.get(1)?.deref())?)?;
            Ok(Obj::new_const(res))
        }))?;

        // (< value cmpr)
        // returns boolean value of expreesion 'value < cmpr'
        self.add_symbol("<", Obj::new_bridge(|env, node| {
            let res = Obj::le(&env.eval(node.get(0)?.deref())?, &env.eval(node.get(1)?.deref())?)?;
            Ok(Obj::new_const(res))
        }))?;

        // (<= value cmpr)
        // returns boolean value of expreesion 'value <= cmpr'
        self.add_symbol("<=", Obj::new_bridge(|env, node| {
            let res = Obj::le_eq(&env.eval(node.get(0)?.deref())?, &env.eval(node.get(1)?.deref())?)?;
            Ok(Obj::new_const(res))
        }))?;

        // (> value cmpr)
        // returns boolean value of expreesion 'value > cmpr'
        self.add_symbol(">", Obj::new_bridge(|env, node| {
            let res = !Obj::le_eq(&env.eval(node.get(0)?.deref())?, &env.eval(node.get(1)?.deref())?)?;
            Ok(Obj::new_const(res))
        }))?;

        // (>= value cmpr)
        // returns boolean value of expreesion 'value >= cmpr'
        self.add_symbol(">=", Obj::new_bridge(|env, node| {
            let res = !Obj::le(&env.eval(node.get(0)?.deref())?, &env.eval(node.get(1)?.deref())?)?;
            Ok(Obj::new_const(res))
        }))?;

        // (loop cond body)
        // loops over body while cond is true
        self.add_symbol("loop", Obj::new_bridge(|env, node| {
            let mut res = Obj::Nil();
            let cond = node.get(0)?;

            while *env.eval(cond.deref())?.is_bool()? {
                res = node.progn(|obj| { env.eval(obj.deref()) })?;
            }
            Ok(res)
        }))?;

        // (lazy obj)
        // returns a lazy 'obj'
        self.add_symbol("lazy", Obj::new_bridge(|_, node| {
            Ok(Obj::new_const(node.get_shared(0)?))
        }))?;

        // (eval obj)
        // returns an evaluated 'obj'
        //  - used to evaluate lazy expressions 
        self.add_symbol("eval", Obj::new_bridge(|env, node| {
            let res = env.eval(node.get(0)?.deref())?;

            match &res {
                Obj::Lazy(lazy) => Ok(env.eval(lazy.borrow().deref())?),
                _ => Ok(res)
            }
        }))?;

        // (if cond if-true if-false)
        // executes 'if-true' if 'cond' is true, 'if-false' otherwise
        self.add_symbol("if", Obj::new_bridge(|env, node| {
            let cond = *env.eval(node.get(0)?.deref())?.is_bool()?;

            if cond {
                env.eval(node.get(1)?.deref())
            } else {
                env.eval(node.get(2)?.deref())
            }
        }))?;

        Ok(())
    }
}