use std::ops::Deref;

use crate::core::{
    env::Env, 
    objects::Obj, 
    err::JtsErr,
    operations::*, 
};

impl Env {
    pub fn arithmetic_lib(&mut self) -> JtsErr {

        self.add_symbol("+", Obj::new_bridge(|_, node| {
            let mut fst = node.shift().get(0).clone();
            node.for_each(|rst| add_to(&mut fst, rst.deref()));
            Ok(fst)
        }))?;

        self.add_symbol("-", Obj::new_bridge(|_, node| {
            let mut fst = node.shift().get(0).clone();
            node.for_each(|rst| sub_to(&mut fst, rst.deref()));
            Ok(fst)
        }))?;

        self.add_symbol("*", Obj::new_bridge(|_, node| {
            let mut fst = node.shift().get(0).clone();
            node.for_each(|rst| mul_to(&mut fst, rst.deref()));
            Ok(fst)
        }))?;

        self.add_symbol("/", Obj::new_bridge(|_, node| {
            let mut fst = node.shift().get(0).clone();
            node.for_each(|rst| div_to(&mut fst, rst.deref()));
            Ok(fst)
        }))
    }
}