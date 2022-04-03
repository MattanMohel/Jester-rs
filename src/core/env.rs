use super::{
    objects::Obj,
    modules::Mod,
    nodes::Node,

    err::{
        JtsErrType::*, 
        JtsErr, 
        AsResult
    },
};

use crate::lex::parser::{
    parse_file, 
    parse_src
};

use std::{
    collections::HashMap,
    cell::RefCell,
    ops::Deref, 
    io::Write,
    rc::Rc, time::{Instant, Duration}, 
};

pub const MAIN: &str = "main";
pub const GEN_SYM: &str = "gensym";
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
    
    pub fn add_symbol_to(&mut self, mod_id: &String, symbol: &String, value: Obj) -> JtsErr {        
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

    pub fn add_symbol(&mut self, symbol: &str, value: Obj) -> JtsErr {        
        self.add_symbol_to(&String::from(PRELUDE), &String::from(symbol), value)?;
        Ok(())
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

        let body = parse_file(self, mod_id, path)?;
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

    pub fn main(&self) -> JtsErr<Shared<Node>> {
        Ok(self.module(&String::from(MAIN))?.deref().borrow().body())
    }

    //////////////////
    /////Run-Time/////
    //////////////////
    
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
        use std::io;

        let mut res = Obj::Nil();
        let mut count: usize = 0;
        let mut time = Duration::new(0, 0);

        loop {
            print!("[{}]>> ", count);
            io::stdout().flush()?;
            count += 1;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim() {
                "--quit" => break,
                "--time" => {
                    println!("completed in: {:?}", time);
                    continue;
                },
                _ => ()
            }

            let body = parse_src(self, &String::from(PRELUDE), &input.trim().to_string())?;

            let start = Instant::now();
            res = self.run(&body)?;
            time = start.elapsed();

            println!("{}", res);
        }

        Ok(res)
    }
}