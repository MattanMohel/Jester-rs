
use std::{
    cell::{RefCell}, 
    rc::Rc, 
    ops::Deref
};

use super::{
    objects::Obj,
    env::Shared,
};

#[derive(Clone)]
pub struct Node{
    pub args: Vec<Shared<Obj>>,
}

impl Default for Node {
    fn default() -> Self {
        Node { args: Vec::new() }
    }
}