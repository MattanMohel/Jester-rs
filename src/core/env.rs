
use maplit::hashmap;

use crate::lex::lexer::to_toks;
use crate::lex::parser::parse_toks;

use super::objects::{Obj, ObjData};
use super::modules::Mod;

use std::cell::RefMut;
use std::ops::Deref;
use std::{
    collections::HashMap,
    fs,

    rc::Rc,
    
    cell::RefCell
};

const GEN_SYM: &str = "gensym";
const PRELUDE: &str = "prelude";

pub struct ObjIn(pub RefCell<Obj>, pub RefCell<ObjData>);

pub struct Env {
    symbols: Vec<Rc<ObjIn>>,
    modules: HashMap<String, Mod>,

    gen_id: usize,
}

impl Env {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
            modules: hashmap! { 
                String::from(PRELUDE) => Mod::new(&String::from(PRELUDE)) 
            },
            gen_id: 0,
        }
    }
    
    pub fn add_symbol_to<T, U>(&mut self, module: T, symbol: U, obj: Obj) 
    where T: Into<String>, U: Into<String> {   
        let module = module.into();
        let symbol = symbol.into();

        assert!(self.symbol_type(&symbol));
        assert! (self.has_module(&module));

        self.symbols.push(Rc::new(
            ObjIn (
                RefCell::new(obj),           
                RefCell::new(ObjData {
                    is_pub:   true,
                    is_const: false,
                    ref_count: 0
                }
            ))
        ));

        let index = self.symbols.len() - 1;

        self.module_mut(&module)
            .unwrap()
            .add_symbol(index, &symbol);    
    }

    pub fn add_symbol<T>(&mut self, symbol: T, obj: Obj) 
    where T: Into<String> {
        self.add_symbol_to(PRELUDE, symbol, obj)
    }


    pub fn new_module<T>(&mut self, name: T) 
    where T: Into<String> {
        let name = name.into();
        assert!( !self.has_module(&name) );

        let (k, v) = Mod::new_key(&name);
        self.modules.insert(k, v);
    }

    pub fn new_module_from_file<T>(&mut self, name: T, path: T) 
    where T: Into<String> {
        let name = name.into();
        assert!( !self.has_module(&name) );

        let (k, v) = Mod::new_key(&name);
        self.modules.insert(k.clone(), v);

        parse_toks(self, &k, &to_toks(&fs::read_to_string(path.into()).unwrap()));
    }

    pub fn gen_symbol_unique(&mut self) -> String {
        self.gen_id += 1;
        format!("{}{}{}{}{}", "__", GEN_SYM, "-", self.gen_id - 1, "__")
    }

    /// Stats and Data

    pub fn has_module<T: Into<String>>(&self, name: T) -> bool {
        self.modules.contains_key(&name.into())
    }

    fn symbol_type(&mut self, symbol: &String) -> bool {
        let beg = &symbol[0..2] == "__";
        let end = &symbol[symbol.len() - 2..] == "__";
        let mid = symbol[2..symbol.len() - 2].parse::<usize>();

        if beg && end && matches!(mid, Ok(n) if n == self.gen_id) {
            self.gen_id += 1;
            true
        } else {
            false
        }
    }

    /// Getters

    pub fn module_mut<T>(&mut self, name: T) -> Option<&mut Mod> 
    where T: Into<String> {
        self.modules.get_mut(&name.into())
    }

    pub fn module<T>(&self, name: T) -> Option<&Mod> 
    where T: Into<String> {
        self.modules.get(&name.into())
    }

    pub fn shared_obj<T>(&mut self, symbol: T) -> Option<Rc<ObjIn>> 
    where T: Into<String> {
        let symbol = symbol.into();

        for module in self.modules.values() {
            if let Some(index) = module.symbol_index(self, &symbol) {
                return Some(self.symbols[index].clone());
            }
        }
        None 
    }

    pub fn obj_mut<T>(&mut self, symbol: T) -> Option<RefMut<'_, Obj>> 
    where T: Into<String> {
        let symbol = symbol.into();

        for module in self.modules.values() {
            if let Some(index) = module.symbol_index(self, &symbol) {
                return Some(self.symbols[index].deref().0.borrow_mut());
            }
        }
        None 
    }
}