
use std::{
    ops::Deref,

    cell::{
        Ref,
        RefMut, RefCell
    }, rc::Rc, 
};

use super::{
    objects::Obj,
    env::{Shared, Env, new_shared}, 
    
    err::{
        JtsErr, 
        ErrType::*
    },
};

#[derive(Clone)]
pub struct Node{
    pub args: Vec<Shared<Obj>>,
}

pub struct NodeIter<'a> {
    pub args: &'a Vec<Shared<Obj>>,
    offset: usize,
    i: usize,
}

impl Default for Node {
    fn default() -> Self {
        Node { args: Vec::new() }
    }
}

impl<'a> IntoIterator for &'a Node {
    type Item = Ref<'a, Obj>;
    type IntoIter = NodeIter<'a>;
    
    fn into_iter(self) -> Self::IntoIter {
        NodeIter {
            args: &self.args,
            offset: 0,
            i: 0,
        }
    }
}

impl Node {
    pub fn get(&self, i: usize) -> JtsErr<Ref<'_, Obj>> {
        match self.args.get(i) {
            Some(obj) => Ok(obj.borrow()),
            None => Err(OutOfBounds)
        }
    }

    pub fn get_mut(&self, i: usize) -> JtsErr<RefMut<'_, Obj>> {
        match self.args.get(i) {
            Some(obj) => Ok(obj.borrow_mut()),
            None => Err(OutOfBounds)
        }    }

    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    pub fn try_collect<F>(&self, mut f: F) -> JtsErr<Self> 
        where F: FnMut(Ref<'_, Obj>) -> JtsErr<Obj>
    {
        let mut err = Ok(());
        let col = self.into_iter()
            .scan(&mut err, |e, obj| {
                match f(obj) {
                    Ok(obj) => Some(new_shared(obj)),
                    Err(err) => {
                        **e = Err(err);
                        None
                    }
                }
            })
            .collect();
        
        err?;
        Ok(Node {args: col})
    } 
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = Ref<'a, Obj>;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        self.args.get(self.offset + self.i - 1)
            .map(|symbol| { symbol.borrow() })
    }
}

impl<'a> NodeIter<'a> {
    pub fn shift(&mut self) -> &mut Self {
        self.offset += 1;
        self
    }

    pub fn get_shared(&self, i: usize) -> JtsErr<Shared<Obj>> {
        match self.args.get(self.offset + i) {
            Some(obj) => Ok(obj.clone()),
            None => Err(OutOfBounds)
        }
    } 

    pub fn get(&self, i: usize) -> JtsErr<Ref<'_, Obj>> {
        match self.args.get(self.offset + i) {
            Some(obj) => Ok(obj.borrow()),
            None => Err(OutOfBounds)
        }
    }

    pub fn get_mut(&self, i: usize) -> JtsErr<RefMut<'_, Obj>> {
        match self.args.get(self.offset + i) {
            Some(obj) => Ok(obj.borrow_mut()),
            None => Err(OutOfBounds)
        }    
    }

    pub fn progn<F1>(&self, mut f1: F1) -> JtsErr<Obj>  
        where F1: FnMut(Ref<'_, Obj>) -> JtsErr<Obj> 
    {
        let bounds = self.args.len().checked_sub(self.offset + 1).unwrap_or(0);

        for i in 0..bounds {
            f1(self.get(i)?)?;
        }

        f1(self.get(bounds)?)
    }

    pub fn progn_then<F1, F2>(&self, mut f1: F1, mut f2: F2) -> JtsErr<Obj>  
        where F1: FnMut(Ref<'_, Obj>) -> JtsErr, F2: FnMut(Ref<'_, Obj>) -> JtsErr<Obj> 
    {
        let bounds = self.args.len().checked_sub(self.offset + 1).unwrap_or(0);

        for i in 0..bounds {
            f1(self.get(i)?)?;
        }

        f2(self.get(bounds)?)
    }
}