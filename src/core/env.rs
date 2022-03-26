use super::{
    objects::Obj,
    modules::Mod,
};

use std::{
    collections::HashMap,
    cell::RefCell,
    rc::Rc, ops::Deref, 
};

const GEN_SYM: &str = "gensym";
const PRELUDE: &str = "prelude";

pub struct Env {
    symbols: Vec<Rc<RefCell<Obj>>>,
    modules: HashMap<String, Rc<RefCell<Mod>>>,
    gen: RefCell<usize>,
}

impl Env {
    pub fn new() -> Self {
        let mut env = Self {
            symbols: Vec::new(),
            modules: HashMap::new(),
            gen: RefCell::new(0),
        };

        env.modules.insert(
            String::from(PRELUDE), 
            Rc::new(RefCell::new(Mod::new())));
        env
    }

    ///////////////////////////
    /////Symbols & Objects/////
    ///////////////////////////
    
    pub fn add_symbol(&mut self, module: &String, symbol: &String, value: Obj) {        
        self.symbols.push(Rc::new(RefCell::new(value)));
        
        match self.modules.get(module) {
            Some(module) => module.deref().borrow_mut()
                .add_symbol(symbol, self.symbols.last().unwrap()),
            
            None => panic!("tried accessing undeclared module!"),
        }
    }

    pub fn unique_symbol(&mut self) -> String {
        (*self.gen.borrow_mut()) += 1;
        format!("__{}-{}__", GEN_SYM, (*self.gen.borrow()) - 1)   
    }

    /////////////////
    /////Modules/////
    /////////////////

    pub fn module(&self, mod_id: &String) -> Option<&RefCell<Mod>> {
        self.modules.get(mod_id)
            .map(|module| {
                module.deref()
            })
    }

    pub fn symbol(&self, symbol: &String) -> Option<Rc<RefCell<Obj>>> {
        self.modules.values()
            .find_map(|module| {
                module.borrow().symbol(symbol)
            })
    }
}