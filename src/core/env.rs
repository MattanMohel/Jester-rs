
use super::objects::{Obj, Node};
use super::modules::Module;
use super::types::TypeId;

pub struct Env {    
    modules: Vec<Module>,
    module_index: usize,

    stackPtrBeg: *mut Node,
    stackPtrCur: *mut Node,
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

    pub fn exec(&mut self) {
        self.stackPtrCur = self.stackPtrBeg;

        while !self.stackPtrBeg.is_null() {
            // eval

            unsafe {
                self.stackPtrCur = (*self.stackPtrCur).next;
            }
        }
    }
}

pub fn newConst<T: TypeId>(val: T) -> Obj {
    Obj::new(val.as_variant())
}