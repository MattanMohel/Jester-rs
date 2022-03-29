use super::{
    objects::Obj,
    modules::Mod,

    err::{
        ErrType::*, 
        JtsErr, 
        AsResult
    },
};

use crate::{
    lex::{
        parser::module_from_file, 
        lexer::to_toks
    },

    prelude::arithmetic::*,
};

use std::{
    collections::HashMap,
    cell::RefCell,
    ops::Deref, 
    rc::Rc, 
    fs, 
};

const MAIN_FN: &str = "main";
const GEN_SYM: &str = "gensym";
const PRELUDE: &str = "prelude";

pub type Shared<T> = Rc<RefCell<T>>;

pub struct Env {
    symbols: Vec<Shared<Obj>>,
    modules: HashMap<String, Shared<Mod>>,
    gen_sym: usize,
    gen_mod: usize,

    curr_unique_sym: String
}

impl Env {
    pub fn new() -> JtsErr<Self> {
        let mut env = Self {
            symbols: Vec::new(),
            modules: HashMap::new(),
            gen_sym: 0,
            gen_mod: 0,
            curr_unique_sym: String::new(),
        };

        // should never error
        env.add_module(&String::from(PRELUDE))?;
        env.gen_unique_symbol();

        env.arithmetic_lib()?;
        env.io_lib()?;

        Ok(env)
    }

    ///////////////////////////
    /////Symbols & Objects/////
    ///////////////////////////
    
    pub fn add_symbol_to(&mut self, mod_id: &String, symbol: &String, value: Obj) -> JtsErr {        
        self.symbols.push(Rc::new(RefCell::new(value)));
     
        if *symbol == self.curr_unique_sym {
            self.gen_unique_symbol();
        } else {
            (!Env::is_unique_symbol_form(symbol)).into_result(DisallowedSymbol)?;
        }
        
        self.modules.get(mod_id).into_result(MissingModule)?
            .deref()
            .borrow_mut()
            .add_symbol(symbol, self.symbols.last().unwrap())
    }

    pub fn add_symbol(&mut self, symbol: &str, value: Obj) -> JtsErr {        
        self.add_symbol_to(&String::from(PRELUDE), &String::from(symbol), value)
    }

    fn gen_unique_symbol(&mut self) {
        self.gen_sym += 1;
        self.curr_unique_sym = format!("__{}-{}__", GEN_SYM, self.gen_sym - 1);
    }

    pub fn is_unique_symbol_form(symbol: &String) -> bool {
        symbol.find(GEN_SYM).is_some()
    }

    pub fn unique_symbol(&self) -> String {
        self.curr_unique_sym.clone()
    }

    /////////////////
    /////Modules/////
    /////////////////

    pub fn add_module(&mut self, mod_id: &String) -> JtsErr {
        self.gen_mod += 1;
        self.modules.insert(mod_id.clone(), Rc::new(RefCell::new(Mod::new(self.gen_mod - 1))))   
            .as_result_rev((), DuplicateModule)
    }

    pub fn add_module_from_file(&mut self, mod_id: &String, path: &String) -> JtsErr {
        let src = fs::read_to_string(path)?;   
        let toks = to_toks(&src);

        module_from_file(self, mod_id, &toks)
    }

    pub fn module(&self, mod_id: &String) -> JtsErr<Shared<Mod>> {
        self.modules.get(mod_id).map(|module| { module.clone() })
            .into_result(MissingModule)
    }

    pub fn symbol(&self, symbol: &String) -> JtsErr<Shared<Obj>> {
        self.modules.values().find_map(|module| { module.borrow().symbol(symbol) })
            .into_result(MissingSymbol)
    }

    fn main_fn(&self) -> JtsErr<Shared<Obj>> {
        return self.symbol(&String::from(MAIN_FN)).map_err(|_| { NoEntry });
    }

    //////////////////
    /////Run-Time/////
    //////////////////
    
    pub fn run(&self) -> JtsErr<Obj> {
        let res = self.main_fn()?.deref().borrow().eval(self);
        Ok(res)
    }
}