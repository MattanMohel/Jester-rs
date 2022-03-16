
use super::env::Env;
use super::objects::Obj;

use std::collections::HashMap;

pub struct Mod {
    name: String,
    symbols: HashMap<String, usize>,
    imports: Vec<usize>,
}

impl Mod {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            symbols: HashMap::new(),
            imports: Vec::new(),
        }
    }

    pub fn add_symbol<T: Into<String>>(&mut self, index: usize, symbol: T) {
        self.symbols.insert(symbol.into(), index);
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

    pub fn symbol_index(&self, env: &Env, symbol: &String) -> Option<usize> {
        if let Some(index) = self.symbols.get(symbol) {
            return Some(*index)
        }

        for module in self.imports.iter() {
            if let Some(index) = env.get_mod_at(*module).symbol_index(env, symbol) {
                return Some(index)
            }
        }

        None
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}