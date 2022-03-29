use super::{
    objects::Obj, 
    env::Shared, 
    err::{
        ErrType::*, 
        AsResult,
        JtsErr
    }, 
};

use std::{
    collections::HashMap, 
    ops::Deref, 
};

pub struct Mod {
    symbols: HashMap<String, Shared<Obj>>,
    imports: Vec<Shared<Mod>>,

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

    pub fn add_import(&mut self, module: &Shared<Mod>) -> JtsErr {
        self.imports.iter()
            .any(|module| { module.deref().borrow().id == module.deref().borrow().id })
            .into_result(DuplicateModule)?;

        self.imports.push(module.clone());
        Ok(())
    }

    pub fn add_symbol(&mut self, symbol: &String, value: &Shared<Obj>) -> JtsErr {
        self.symbols.contains_key(symbol).as_result_rev((), DuplicateSymbol)?;
        self.symbols.insert(symbol.clone(), value.clone());
        Ok(())
    }

    pub fn symbol(&self, symbol: &String) -> Option<Shared<Obj>> {
        match self.symbols.get(symbol) {
            Some(symbol) => Some(symbol.clone()),

            None => self.imports.iter()
                .find_map(|module| { 
                    module.deref().borrow().symbols.get(symbol)
                        .map(|symbol| { symbol.clone() }) 
                })  
        }
    }
}