
use std::{
    cell::Ref, 
    rc::Rc, 
    ops::Deref
};

use super::{
    env::ObjIn,
    objects::Obj,
};

#[derive(Clone)]
pub struct Node(pub Vec<Rc<ObjIn>>);

impl<'a> IntoIterator for &'a Node {
    type Item = Ref<'a, Obj>;
    type IntoIter = NodeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIter {
            args: &self.0,
            index: 0
        }
    }
}

impl Node {
    pub fn get<'a>(&'a self, i: usize) -> Ref<'a, Obj> {
        self.0[i].deref().0.borrow()
    }
}

pub struct NodeIter<'a> {
    pub args: &'a Vec<Rc<ObjIn>>,
    pub index: usize,
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = Ref<'a, Obj>;

    fn next(&mut self) -> Option<Self::Item> {
        self.args.get(self.index).map(|cell| {
            cell.deref().0.borrow()
        })
    }
}

impl<'a> NodeIter<'a> {
    pub fn get(&self, i: usize) -> Ref<'a, Obj> {
        self.args[i].deref().0.borrow()
    }
}