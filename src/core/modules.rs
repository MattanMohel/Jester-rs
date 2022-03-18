
use super::env::Env;

use std::collections::HashMap;

pub struct Mod {
    symbols: HashMap<String, usize>,
    modules: Vec<String>,
}

impl Mod {
    pub fn new(name: &String) -> Self {
        Self {
            symbols: HashMap::new(),
            modules: vec![name.clone()],
        }
    }

    pub fn new_key(name: &String) -> (String, Self) {
        let s = Self {
            symbols: HashMap::new(),
            modules: vec![name.clone()],
        };

        (name.clone(), s)
    }

    pub fn add_symbol(&mut self, index: usize, symbol: &String) {
        self.symbols.insert(symbol.into(), index);
    }

    pub fn has_symbol(&self, env: &Env, symbol: &String) -> bool {
        self.modules
            .iter()
            .any(|name| { env.module(name).unwrap().symbols.contains_key(symbol) })
    }

    pub fn symbol_index(&self, env: &Env, symbol: &String) -> Option<usize> {
        self.modules
            .iter()
            .find_map(|name| { env.module(name).unwrap().symbols.get(symbol).map(|i| *i ) })
    }
}