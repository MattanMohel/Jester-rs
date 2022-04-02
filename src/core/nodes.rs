
use std::{
    ops::Deref,

    cell::{
        Ref,
        RefMut
    }, 
};

use super::{
    objects::Obj,
    env::Shared, 
    
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
            Some(obj) => Ok(obj.deref().borrow()),
            None => Err(OutOfBounds)
        }
    }

    pub fn get_mut(&self, i: usize) -> JtsErr<RefMut<'_, Obj>> {
        match self.args.get(i) {
            Some(obj) => Ok(obj.deref().borrow_mut()),
            None => Err(OutOfBounds)
        }    }

    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = Ref<'a, Obj>;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        self.args.get(self.offset + self.i - 1)
            .map(|symbol| { symbol.deref().borrow() })
    }
}

impl<'a> NodeIter<'a> {
    pub fn shift(&mut self) -> &mut Self {
        self.offset += 1;
        self
    }

    pub fn get(&self, i: usize) -> JtsErr<Ref<'_, Obj>> {
        match self.args.get(self.offset + i) {
            Some(obj) => Ok(obj.deref().borrow()),
            None => Err(OutOfBounds)
        }
    }

    pub fn get_mut(&self, i: usize) -> JtsErr<RefMut<'_, Obj>> {
        match self.args.get(self.offset + i) {
            Some(obj) => Ok(obj.deref().borrow_mut()),
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