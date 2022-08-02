use std::ops::Deref;

use crate::core::{
    env::Env, 
    objects::Obj, 
    err::JtsErr,
};

impl Env {
    pub fn arith_lib(&mut self) -> JtsErr {
        // (+ body)
        // calculates the sum of all the elements of body
        self.add_symbol("+", Obj::new_bridge(|env, node| {
            let mut fst = env.eval(&node.get(0)?.deref())?;
            node.shift()?;

            for rst in node {
                fst.add(env.eval(rst.deref())?)?;
            }
            Ok(fst)
        }))?;

        // (- body)
        // calculates the difference of all the elements of body
        self.add_symbol("-", Obj::new_bridge(|env, node| {
            let mut fst = env.eval(&node.get(0)?.deref())?;
            node.shift()?;

            for rst in node {
                fst.sub(env.eval(rst.deref())?)?;
            }
            Ok(fst)
        }))?;

        // (* body)
        // calculates the product of all the elements of body
        self.add_symbol("*", Obj::new_bridge(|env, node| {
            let mut fst = env.eval(&node.get(0)?.deref())?;
            node.shift()?;

            for rst in node {
                fst.mul(env.eval(rst.deref())?)?;
            }
            Ok(fst)
        }))?;


        // (/ body)
        // calculates the quotient of all the elements of body
        self.add_symbol("/", Obj::new_bridge(|env, node| {
            let mut fst = env.eval(&node.get(0)?.deref())?;
            node.shift()?;

            for rst in node {
                fst.div(env.eval(rst.deref())?)?;
            }
            Ok(fst)
        }))?;

        // (+ body)
        // calculates the modulos of all the elements of body
        self.add_symbol("%", Obj::new_bridge(|env, node| {
            let mut fst = env.eval(&node.get(0)?.deref())?;
            node.shift()?;

            for rst in node {
                fst.modulos(env.eval(rst.deref())?)?;
            }
            Ok(fst)
        }))?;

        Ok(())
    }
}