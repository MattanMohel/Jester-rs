use std::ops::Deref;

use crate::core::{
    objects::Obj,
    err::JtsErr,
    
    env::{
        Env, 
        new_shared
    }, 
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

        // (replace index element list)
        // sets the element at index to a new element
        //  - returns the replaced element
        self.add_symbol("replace", Obj::new_bridge(|env, node| {
            let index = env.eval(node.get(0)?.deref())?.is_int()? as usize;
            let elem = env.eval(node.get(1)?.deref())?;

            let list = node.get_mut(2)?;
            let list = list.is_node()?;

            let replace = list.get(index)?.clone();
            list.get_mut(index as usize)?.set(&elem);
            
            Ok(replace)
        }))?;

        // (append elem list)
        // appends a given element to the end of a list
        self.add_symbol("append", Obj::new_bridge(|env, node| {
            let elem = env.eval(node.get(0)?.deref())?;
            node.get_mut(1)?.is_node_mut()?.args.push(new_shared(elem.clone()));

            Ok(elem)
        }))?;

        // (prepend elem list)
        // prepend a given element to the end of a list
        self.add_symbol("prepend", Obj::new_bridge(|env, node| {
            let elem = env.eval(node.get(0)?.deref())?;
            node.get_mut(1)?.is_node_mut()?.args.insert(0, new_shared(elem.clone()));
            
            Ok(elem)
        }))?;

        // (insert index elem list)
        // inserts a given element to the list at the given index
        //  - pushes all elements to the right
        self.add_symbol("insert", Obj::new_bridge(|env, node| {
            let index = env.eval(node.get(0)?.deref())?.is_int()?;
            let elem = env.eval(node.get(1)?.deref())?;
            node.get_mut(2)?.is_node_mut()?.args.insert(index as usize, new_shared(elem.clone()));
            
            Ok(elem)
        }))?;

        // (remove index list)
        // removes element from list at the given index
        //  - pushes all elements to the left
        //  - returns the removed element
        self.add_symbol("remove", Obj::new_bridge(|env, node| {
            let index = env.eval(node.get(0)?.deref())?.is_int()?;
            node.get_mut(1)?.is_node_mut()?.remove(index as usize)
        }))?;

        // (len list)
        // returns the length of a list
        self.add_symbol("len", Obj::new_bridge(|env, node| {
            let len = env.eval(node.get(0)?.deref())?.is_node()?.len();
            Ok(Obj::new_const(len as u64))
        }))?;

        Ok(())
    }
}