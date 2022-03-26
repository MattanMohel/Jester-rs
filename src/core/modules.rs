
use super::{
    objects::Obj, 
    env::{Env, Shared}, 
};

use std::{
    collections::HashMap, 
    cell::RefCell, 
    rc::Rc, 
};

pub struct Mod {
    symbols: HashMap<String, Shared<Obj>>,
    imports: Vec<Rc<RefCell<Mod>>>,
}

impl Mod {
    pub(crate) fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            imports: Vec::new(),
        }
    }

    pub fn add_import(&mut self, module: &Shared<Mod>) {
        self.imports.push(module.clone());
    }

    pub fn add_symbol(&mut self, symbol: &String, value: &Shared<Obj>) -> Shared<Obj> {
        self.symbols.insert(
            symbol.clone(), 
            value.clone()).unwrap()
    }

    pub fn symbol(&self, symbol: &String) -> Option<Shared<Obj>> {
        match self.symbols.get(symbol) {
            Some(symbol) => Some(symbol.clone()),

            None => self.imports.iter()
                .find_map(|module| { 
                    module.borrow().symbols.get(symbol)
                        .map(|symbol| { symbol.clone() }) 
                })  
        }
    }
}