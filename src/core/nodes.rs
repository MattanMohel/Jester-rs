
use std::{
    cell::{RefCell}, 
    rc::Rc, 
    ops::Deref
};

use super::{
    objects::Obj,
};

#[derive(Clone)]
pub struct Node{
    pub args: Vec<Rc<RefCell<Obj>>>,
}

pub struct NodeIter<'a> {
    pub args: &'a Vec<Rc<RefCell<Obj>>>,
    pub index: usize,
}

impl<'a> IntoIterator for &'a Node {
    type Item = &'a RefCell<Obj>;
    type IntoIter = NodeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIter {
            args: &self.args,
            index: 0 
        }
    }
}


impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a RefCell<Obj>;

    fn next(&mut self) -> Option<Self::Item> {
        self.args.get(self.index)
            .map(|obj| {
                obj.deref()
            })
    }
}

impl<'a> NodeIter<'a> {
    pub fn get(&self, i: usize) -> &'a RefCell<Obj> {
        self.args[i].deref()
    }
}