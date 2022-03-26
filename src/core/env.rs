use crate::lex::{parser::module_from_file, lexer::to_toks};

use super::{
    objects::Obj,
    modules::Mod,
};

use std::{
    collections::HashMap,
    cell::RefCell,
    rc::Rc, ops::Deref, fs, 
};

const GEN_SYM: &str = "gensym";
const PRELUDE: &str = "prelude";

pub type Shared<T> = Rc<RefCell<T>>;

pub struct Env {
    symbols: Vec<Shared<Obj>>,
    modules: HashMap<String, Shared<Mod>>,
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
    
    pub fn add_symbol(&mut self, module: &String, symbol: &String, value: Obj) -> Shared<Obj> {        
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

    pub fn add_module(&mut self, mod_id: &String, module: Mod) -> Shared<Mod> {
        self.modules.insert(
            mod_id.clone(), 
            Rc::new(RefCell::new(module))).unwrap()
    }

    pub fn add_module_from_file(&mut self, mod_id: &String, path: &String) {
        let src = fs::read_to_string(path)
            .expect(&format!("path {} is invalid - adding module to file", path));
        
        let toks = to_toks(&src);

        module_from_file(self, mod_id, &toks);
    }

    pub fn module(&self, mod_id: &String) -> Option<Shared<Mod>> {
        self.modules.get(mod_id)
            .map(|module| {
                module.clone()
            })
    }

    pub fn symbol(&self, symbol: &String) -> Option<Shared<Obj>> {
        self.modules.values()
            .find_map(|module| {
                module.borrow().symbol(symbol)
            })
    }
}