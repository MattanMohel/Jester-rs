use super::{
    objects::Obj,
    modules::Mod,

    err::{
        ParseErrType::*, 
        ParseErr, 
        AsResult
    },
};

use crate::lex::{
    parser::module_from_file, 
    lexer::to_toks
};

use std::{
    collections::HashMap,
    cell::RefCell,
    ops::Deref, 
    rc::Rc, 
    fs, 
};

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
    pub fn new() -> Self {
        let mut env = Self {
            symbols: Vec::new(),
            modules: HashMap::new(),
            gen_sym: 0,
            gen_mod: 0,
            curr_unique_sym: String::new(),
        };

        // should never error
        env.add_module(&String::from(PRELUDE)).unwrap();
        env.gen_unique_symbol();
        env
    }

    ///////////////////////////
    /////Symbols & Objects/////
    ///////////////////////////
    
    pub fn add_symbol(&mut self, mod_id: &String, symbol: &String, value: Obj) -> ParseErr {        
        self.symbols.push(Rc::new(RefCell::new(value)));
     
        if *symbol == self.curr_unique_sym {
            self.gen_unique_symbol();
        } else {
            (!Env::is_unique_symbol_form(symbol)).into_result(DisSym)?;
        }
        
        self.modules.get(mod_id).into_result(NonMod)?
            .deref()
            .borrow_mut()
            .add_symbol(symbol, self.symbols.last().unwrap())
    }

    fn gen_unique_symbol(&mut self) {
        self.gen_sym += 1;
        self.curr_unique_sym = format!("__{}-{}__", GEN_SYM, self.gen_sym - 1);
    }

    pub fn is_unique_symbol_form(symbol: &String) -> bool {
        symbol.find("gensym").is_some()
    }

    pub fn unique_symbol(&self) -> String {
        self.curr_unique_sym.clone()
    }

    /////////////////
    /////Modules/////
    /////////////////

    pub fn add_module(&mut self, mod_id: &String) -> ParseErr {
        self.modules.insert(mod_id.clone(), Rc::new(RefCell::new(Mod::new(self.gen_mod))))   
            .as_result_rev((), DupMod)
    }

    pub fn add_module_from_file(&mut self, mod_id: &String, path: &String) -> ParseErr {
        let src = fs::read_to_string(path)?;   
        let toks = to_toks(&src);

        module_from_file(self, mod_id, &toks)
    }

    pub fn module(&self, mod_id: &String) -> ParseErr<Shared<Mod>> {
        self.modules.get(mod_id).map(|module| { module.clone() })
            .into_result(NonMod)
    }

    pub fn symbol(&self, symbol: &String) -> ParseErr<Shared<Obj>> {
        println!("getting symbol {}", symbol);
        self.modules.values().find_map(|module| { module.borrow().symbol(symbol) })
            .into_result(NonSym)
    }
}