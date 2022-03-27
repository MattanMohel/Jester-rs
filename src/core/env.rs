use crate::lex::{
    parser::module_from_file, 
    lexer::to_toks
};

use super::{
    objects::Obj,
    modules::Mod,
    err::{ParseErr, AsResult},
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
    gen_mod: usize,
}

impl Env {
    pub fn new() -> Self {
        let mut env = Self {
            symbols: Vec::new(),
            modules: HashMap::new(),
            gen: RefCell::new(0),
            gen_mod: 0,
        };

        // should never error
        env.add_module(&String::from(PRELUDE)).unwrap();
        env
    }

    ///////////////////////////
    /////Symbols & Objects/////
    ///////////////////////////
    
    pub fn add_symbol(&mut self, mod_id: &String, symbol: &String, value: Obj) -> Result<(), ParseErr> {        
        self.symbols.push(Rc::new(RefCell::new(value)));
        
        self.modules.get(mod_id)
            .into_result(ParseErr::NonMod(mod_id.clone()))?
            .deref().borrow_mut()
            
            .add_symbol(symbol, self.symbols.last().unwrap())
    }

    pub fn unique_symbol(&self) -> String {
        (*self.gen.borrow_mut()) += 1;
        format!("__{}-{}__", GEN_SYM, (*self.gen.borrow()) - 1)   
    }

    /////////////////
    /////Modules/////
    /////////////////

    pub fn add_module(&mut self, mod_id: &String) -> Result<(), ParseErr> {
        self.modules.insert(
            mod_id.clone(), Rc::new(RefCell::new(Mod::new(self.gen_mod))))
            
            .as_result_rev((), ParseErr::DupMod(mod_id.clone()))
    }

    pub fn add_module_from_file(&mut self, mod_id: &String, path: &String) -> Result<(), ParseErr> {
        let src = fs::read_to_string(path)?;   
        let toks = to_toks(&src);

        module_from_file(self, mod_id, &toks)
    }

    pub fn module(&self, mod_id: &String) -> Option<Shared<Mod>> {
        self.modules.get(mod_id)
            .map(|module| { module.clone() })
    }

    pub fn symbol(&self, symbol: &String) -> Option<Shared<Obj>> {
        self.modules.values()
            .find_map(|module| { module.borrow().symbol(symbol) })
    }
}