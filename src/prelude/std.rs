use std::{ops::Deref, borrow::Borrow};

use crate::core::{
    env::Env, 
    objects::Obj, 
    err::JtsErr,
    functions::FnNative, 
};

impl Env {
    pub fn std_lib(&mut self) -> JtsErr {
        self.add_symbol("T", Obj::new_const(true))?;
        self.add_symbol("F", Obj::new_const(false))?;

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

        Ok(())
    }
}