use std::ops::Deref;

use crate::core::{
    env::Env, 
    objects::Obj, 
    err::JtsErr,
};

impl Env {
    pub fn arith_lib(&mut self) -> JtsErr {
        self.add_symbol("+", Obj::new_bridge(|env, node| {
            let mut fst = node.get(0)?.clone();

            for rst in node.shift() {
                fst.add(env.eval(rst.deref())?)?;
            }
            Ok(fst)
        }))?;

        self.add_symbol("-", Obj::new_bridge(|env, node| {
            let mut fst = node.get(0)?.clone();

            for rst in node.shift() {
                fst.sub(env.eval(rst.deref())?)?;
            }
            Ok(fst)
        }))?;


        self.add_symbol("*", Obj::new_bridge(|env, node| {
            let mut fst = node.get(0)?.clone();

            for rst in node.shift() {
                fst.mul(env.eval(rst.deref())?)?;
            }
            Ok(fst)
        }))?;


        self.add_symbol("/", Obj::new_bridge(|env, node| {
            let mut fst = node.get(0)?.clone();

            for rst in node.shift() {
                fst.div(env.eval(rst.deref())?)?;
            }
            Ok(fst)
        }))?;

        self.add_symbol("%", Obj::new_bridge(|env, node| {
            let mut fst = node.get(0)?.clone();

            for rst in node.shift() {
                fst.modulos(env.eval(rst.deref())?)?;
            }
            Ok(fst)
        }))?;

        Ok(())
    }
}