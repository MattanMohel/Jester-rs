
use super::env::Env;
use super::objects::Obj;

use std::collections::HashMap;

pub struct Module {
    name: String,
    symbols: HashMap<String, usize>,
    imports: Vec<usize>,
}

impl Module {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            symbols: HashMap::new(),
            imports: Vec::new(),
        }
    }

    pub(in super) fn add_symbol(&mut self, symbol: &String, index: usize) {
        self.symbols.insert(symbol.clone(), index);
    }

    pub fn has_symbol(&self, env: &Env, symbol: &String) -> bool {
        if self.symbols.contains_key(symbol) {
            return true
        }
        
        for module in self.imports.iter() {
            if env.get_mod_at(*module).has_symbol(env, symbol) {
                return true
            }
        }

        false
    }

    // returns the index of a given symbol

    pub fn get_symbol_index(&self, symbol: &String) -> Option<usize> {
        if let Some(index) = self.symbols.get(symbol) {
            return Some(*index)
        }

        None
    }

    // returns the object corresponding to a given symbol
    
    pub fn get_symbol<'a>(&self, env: &'a mut Env, symbol: &String) -> Option<&'a mut Obj> {
        if let Some(index) = self.symbols.get(symbol) {
            return Some(env.get_obj_at_mut(*index))
        }

        for module in self.imports.iter() {
            if let Some(obj) = env.get_mod_at(*module).get_symbol_index(symbol) {
                return Some(env.get_obj_at_mut(obj))
            }
        }

        None
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}