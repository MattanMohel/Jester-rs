
use super::{
    objects::Obj, 
};

use std::{
    collections::HashMap, 
    cell::RefCell, 
    rc::Rc, 
};

pub struct Mod {
    symbols: HashMap<String, Rc<RefCell<Obj>>>,
    imports: Vec<Rc<RefCell<Mod>>>,
}

impl Mod {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            imports: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: &String, value: &Rc<RefCell<Obj>>) {
        self.symbols.insert(symbol.clone(), value.clone());
    }

    pub fn symbol(&self, symbol: &String) -> Option<Rc<RefCell<Obj>>> {
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