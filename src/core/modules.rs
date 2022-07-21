use super::{
    objects::Obj, 
    env::{Shared, new_shared}, 
    nodes::Node,

    err::{
        JtsErrType::*, 
        AsResult,
        JtsErr
    }, 
};

use std::{
    collections::HashMap, 
    ops::Deref, 
    rc::Rc, 
};

pub struct Mod {
    pub symbols: HashMap<String, Shared<Obj>>,
    imports: Vec<Shared<Mod>>,
    body: Shared<Node>, 

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
            body: Rc::default(),
            id: id,
        }
    }

    pub fn add_import(&mut self, module: &Shared<Mod>) -> JtsErr {
        self.imports.iter()
            .all(|iter| { module.deref().borrow().id != iter.deref().borrow().id })
            .into_result(DuplicateModule)?;

        self.imports.push(module.clone());
        Ok(())
    }

    pub fn add_symbol(&mut self, symbol: &String, value: &Shared<Obj>) -> JtsErr {
        self.symbols.contains_key(symbol).as_result_rev((), DuplicateSymbol)?;
        self.symbols.insert(symbol.clone(), value.clone());
        Ok(())
    }

    pub fn add_body(&mut self, body: Node) {
        self.body = new_shared(body);
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

    pub fn symbol_id(&self, obj: &Shared<Obj>) -> Option<String> {
        for key in &self.symbols {
            if obj.deref().as_ptr() == key.1.as_ptr() {
                return Some(key.0.clone())
            }     
        }

        None
    }

    pub fn body(&self) -> Shared<Node> {
        self.body.clone()
    }
}