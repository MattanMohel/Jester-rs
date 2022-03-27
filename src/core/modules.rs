
use crate::core::err::AsResult;

use super::{
    objects::Obj, 
    env::{Env, Shared}, err::ParseErr, 
};

use std::{
    collections::HashMap, 
    cell::RefCell, 
    rc::Rc, 
};

pub struct Mod {
    symbols: HashMap<String, Shared<Obj>>,
    imports: Vec<Rc<RefCell<Mod>>>,

    id: usize,
}

impl PartialEq for Mod {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Mod {
    pub(crate) fn new(id: usize) -> Self {
        Self {
            symbols: HashMap::new(),
            imports: Vec::new(),
            id: id,
        }
    }

    pub fn add_import(&mut self, module: &Shared<Mod>) {
        self.imports.push(module.clone());
    }

    pub fn add_symbol(&mut self, symbol: &String, value: &Shared<Obj>) -> Result<(), ParseErr> {
        self.symbols.insert(symbol.clone(), value.clone())
            .as_result_rev((), ParseErr::DupSym(symbol.clone()))
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