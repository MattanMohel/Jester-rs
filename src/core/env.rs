
use std::collections::HashMap;

use super::objects::{Obj, Node};
use super::modules::Module;
use super::types::{Type, TypeId};
use super::functions::NativeFn;
use crate::util::mem_pool::MemPool;

const NODE_POOL_SZ: usize = 1000;

pub struct Env {    
    modules: Vec<Module>,
    prelude: HashMap<String, Obj>,

    node_pool: MemPool<Node, NODE_POOL_SZ>,

    // runtime states
    pub eval: bool,
}

impl Env {
    pub fn new() -> Env {
        let node_pool = MemPool::new(|node: *mut Node| unsafe {
            // (*node).next = std::ptr::null_mut();
            // (*node).val  = std::ptr::null_mut();
            node
        });

        Env { 
            modules: Vec::new(), 
            prelude: HashMap::new(), 
            node_pool: node_pool,
            eval: false,
        }
    }

    pub fn has(&mut self, symbol: &String) -> bool {
        for module in self.modules.iter() {
            if module.has(symbol) {
                return true
            }
        }
        
        false    
    }
    
    pub fn get(&mut self, symbol: &String) -> Option<&mut Obj> {
        for module in self.modules.iter_mut() {
            if let Some(obj) = module.get(symbol) {
                return Some(obj)
            }
        }

        None
    }

    pub fn add_symbol(&mut self, symbol: &str, obj: Obj) {
        self.prelude.insert(symbol.to_string(), obj.clone());
    }

    pub fn add_lib(&mut self, lib: fn(&mut Env)) {
        lib(self);
    }

    pub fn new_node(&mut self) -> *mut Node {
        self.node_pool.acquire()
    }

    pub fn free_node(&mut self, elem: *mut Node) {
        self.node_pool.release(elem);
    }

    pub fn with_eval(&mut self, eval: bool) -> &mut Env {
        self.eval = eval;
        self
    }
}

pub fn new_const<T: TypeId>(val: T) -> Obj {
    Obj::new(val.as_variant())
}

pub fn new_native(native: fn(&mut Env, Node) -> Obj) -> Obj {
    Obj::new(Type::Native(NativeFn::new(native)))
}