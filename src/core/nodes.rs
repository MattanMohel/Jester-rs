
use std::{
    cell::{RefCell, Ref, RefMut}, 
    rc::Rc, 
    ops::Deref, iter::Peekable
};

use super::{
    objects::Obj,
    env::Shared,
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
    pub fn get(&self, i: usize) -> Ref<'_, Obj> {
        self.args[i].deref().borrow()
    }

    pub fn get_mut(&self, i: usize) -> RefMut<'_, Obj> {
        self.args[i].deref().borrow_mut()
    }

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

    pub fn get(&self, i: usize) -> Ref<'_, Obj> {
        self.args[self.offset + i].deref().borrow()
    }

    pub fn get_mut(&self, i: usize) -> RefMut<'_, Obj> {
        self.args[self.offset + i].deref().borrow_mut()
    }
}