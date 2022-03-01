
use super::objects::{Obj, Node};
use super::modules::Module;
use super::types::{NativeFn, Type, TypeId};
use crate::util::mem_pool::MemPool;

const NODE_POOL_SZ: usize = 1000;

pub struct Env {    
    modules: Vec<Module>,
    module_index: usize,

    node_pool: MemPool<Node, NODE_POOL_SZ>,
}

impl Env {
    pub fn has(&mut self, symbol: &String) -> bool {
        for module in self.modules.iter() {
            if module.has(symbol) {
                return true
            }
        }
        
        false    
    }
    
    pub fn add(&mut self, symbol: &String, obj: Obj) {
        if !self.modules[self.module_index].add(symbol, obj) {
            panic!("tried to add already existing symbol to module")
        }
    }
    
    pub fn get(&mut self, symbol: &String) -> &mut Obj {
        if let Some(obj) = self.modules[self.module_index].get(symbol) {
            return obj
        }
        
        panic!("tried to query non-declared symbol from odule")
    }

    pub fn new_node(&mut self) -> *mut Node {
        self.node_pool.acquire()
    }

    pub fn free_node(&mut self, elem: *mut Node) {
        self.node_pool.release(elem);
    }
}

pub fn new_const<T: TypeId>(val: T) -> Obj {
    Obj::new(val.as_variant())
}

pub fn new_native(native: NativeFn) -> Obj {
    Obj::new(Type::Native(native))
}