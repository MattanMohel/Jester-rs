use std::ops::Deref;

use crate::core::{
    env::Env, 
    objects::Obj, 
    err::JtsErr,
    operations::*, 
};

impl Env {
    pub fn arithmetic_lib(&mut self) -> JtsErr {

        self.add_symbol("+", Obj::new_bridge(|env, node| {
            let mut fst = node.get(0).clone();

            for rst in node.shift() {
                fst.add(rst.eval(env)?);
            }
            Ok(fst)
        }))?;

        self.add_symbol("-", Obj::new_bridge(|env, node| {
            let mut fst = node.get(0).clone();

            for rst in node.shift() {
                fst.sub(rst.eval(env)?);
            }
            Ok(fst)
        }))?;


        self.add_symbol("*", Obj::new_bridge(|env, node| {
            let mut fst = node.get(0).clone();

            for rst in node.shift() {
                fst.mul(rst.eval(env)?);
            }
            Ok(fst)
        }))?;


        self.add_symbol("/", Obj::new_bridge(|env, node| {
            let mut fst = node.get(0).clone();

            for rst in node.shift() {
                fst.div(rst.eval(env)?);
            }
            Ok(fst)
        }))?;

        Ok(())
    }
}