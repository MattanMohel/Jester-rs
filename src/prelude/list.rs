use std::{ops::Deref, cell::RefCell, rc::Rc};

use crate::core::{
    env::Env, 
    objects::Obj, 
    err::JtsErr,
};

impl Env {
    pub fn list_lib(&mut self) -> JtsErr {

        // (nth index list)
        // returns the nth element of a given list
        self.add_symbol("nth", Obj::new_bridge(|env, node| {
            let index = node.get(0)?.is_int()?;
            let res = env.eval(node.get_mut(1)?
                .is_node()?
                .get(index as usize)?
                .deref())?;

            Ok(res)
        }))?;

        // (append elem list)
        // appends a given element to the end of a list
        self.add_symbol("append", Obj::new_bridge(|env, node| {
            let elem = env.eval(node.get(0)?.deref())?;
            node.get_mut(1)?.is_node_mut()?.args.push(Rc::new(RefCell::new(elem.clone())));

            Ok(elem)
        }))?;

        // (prepend elem list)
        // prepend a given element to the end of a list
        self.add_symbol("prepend", Obj::new_bridge(|env, node| {
            let elem = env.eval(node.get(0)?.deref())?;
            node.get_mut(1)?.is_node_mut()?.args.insert(0, Rc::new(RefCell::new(elem.clone())));
            
            Ok(elem)
        }))?;

        // (insert index elem list)
        // inserts a given element to the list at the given index
        //  - pushes all elements to the right
        self.add_symbol("insert", Obj::new_bridge(|env, node| {
            let index = env.eval(node.get(0)?.deref())?.is_int()?;
            let elem = env.eval(node.get(1)?.deref())?;
            node.get_mut(2)?.is_node_mut()?.args.insert(index as usize, Rc::new(RefCell::new(elem.clone())));
            
            Ok(elem)
        }))?;

        Ok(())
    }
}