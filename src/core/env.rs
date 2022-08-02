use super::{
    objects::Obj,
    modules::Mod,
    nodes::Node,

    err::{
        JtsErrType::*, 
        JtsErr, 
        AsResult
    }, 
    
    repl::Repl,
};

use crate::lex::parser::Parser;

use std::{
    collections::HashMap,
    cell::RefCell,
    ops::Deref, 
    rc::Rc, 
};

pub const MAIN: &str = "main";
pub const GEN_SYM: &str = "GEN";
pub const PRELUDE: &str = "prelude";

pub type Shared<T> = Rc<RefCell<T>>;

pub fn new_shared<T>(v: T) -> Shared<T> {
    Rc::new(RefCell::new(v))
}

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

        env.gen_unique_symbol();

        // create and populate prelude module
        env.add_module(&String::from(PRELUDE), false)?;
        env.arith_lib()?;
        env.io_lib()?;
        env.list_lib()?;
        env.std_lib()?;

        Ok(env)
    }

    ///////////////////////////
    /////Symbols & Objects/////
    ///////////////////////////
    
    pub fn add_symbol(&mut self, symbol: &str, value: Obj) -> JtsErr<Shared<Obj>> {        
        self.add_symbol_to(&String::from(PRELUDE), &String::from(symbol), value)
    }
    
    pub fn add_symbol_to(&mut self, mod_id: &String, symbol: &String, value: Obj) -> JtsErr<Shared<Obj>> {        
        self.symbols.push(new_shared(value));
     
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

    fn gen_unique_symbol(&mut self) {
        self.gen_sym += 1;
        self.curr_unique_sym = format!("{}#{}", GEN_SYM, self.gen_sym - 1);
    }

    pub fn is_unique_symbol_form(symbol: &String) -> bool {
        symbol.find(GEN_SYM).is_some() &&
        symbol[GEN_SYM.len() + 1..].parse::<usize>().is_ok()
    }

    pub fn unique_symbol(&self) -> String {
        self.curr_unique_sym.clone()
    }
    
    pub unsafe fn generate_symbol(&self, value: Obj) -> JtsErr<Shared<Obj>> {      
        let ptr = (self as *const Self) as *mut Self;
            
        match ptr.as_mut() {
            Some(mut_self) => {
                let symbol = self.unique_symbol();
                mut_self.gen_unique_symbol();

                mut_self.symbols.push(new_shared(value));
     
                mut_self.modules.get(&PRELUDE.to_string()).into_result(MissingModule)?
                    .deref()
                    .borrow_mut()
                    .add_symbol(&symbol, self.symbols.last().unwrap())
            },
            None => Err(UninitEnv)
        } 
    }

    /////////////////
    /////Modules/////
    /////////////////

    pub fn add_module(&mut self, mod_id: &String, prelude: bool) -> JtsErr {
        self.gen_mod += 1;

        let mut module = Mod::new(self.gen_mod);
        if prelude {
            module.add_import(&self.module(&String::from(PRELUDE))?)?;
        }

        self.modules.insert(mod_id.clone(), new_shared(module))   
            .as_result_rev((), DuplicateModule)
    }

    pub fn add_module_from_file(&mut self, mod_id: &String, path: &String) -> JtsErr {
        if !self.modules.contains_key(mod_id) {
            self.add_module(mod_id, true)?; 
        }

        let body = Parser::from_file(path)?.parse_tokens(self, mod_id)?;
        self.module(mod_id)?.borrow_mut().add_body(body);

        Ok(())
    }

    pub fn module(&self, mod_id: &String) -> JtsErr<Shared<Mod>> {
        self.modules.get(mod_id).map(|module| { module.clone() })
            .into_result(MissingModule)
    }

    pub fn symbol(&self, symbol: &String) -> JtsErr<Shared<Obj>> {
        self.modules.values().find_map(|module| { module.deref().borrow().symbol(symbol) })
            .into_result(MissingSymbol)
    }

    pub fn symbol_id(&self, obj: &Shared<Obj>) -> Option<String> {
        self.modules.values().find_map(|module| { module.borrow().symbol_id(obj) })
    }

    pub fn main(&self) -> JtsErr<Shared<Node>> {
        Ok(self.module(&String::from(MAIN))?.deref().borrow().body())
    }

    //////////////////
    /////Run-Time/////
    //////////////////
    /// 
    pub fn add_src(&mut self, src: &str) -> JtsErr<Obj> {
        let body = Parser::from_string(&String::from(src))?.parse_tokens(self, &String::from(PRELUDE))?;
        self.run(&body)
    }
    
    pub fn run(&self, node: &Node) -> JtsErr<Obj> {
        node.into_iter()
            .progn(|obj|{ self.eval(obj.deref()) })
    }

    pub fn run_main(&self) -> JtsErr<Obj> {
        self.main()?
            .deref()
            .borrow()
            .into_iter()
            .progn(|obj|{ self.eval(obj.deref()) })
    }

    pub fn run_repl(&mut self) -> JtsErr<Obj> {
        let mut repl = Repl::new();
        repl.run(self)
    }
}