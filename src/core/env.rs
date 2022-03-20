
use maplit::hashmap;

use crate::lex::lexer::to_toks;
use crate::lex::parser::parse_toks;

use super::objects::{Obj, ObjData};
use super::modules::Mod;

use std::collections::HashMap;
use std::fs;

const GEN_SYM: &str = "gensym";
const PRELUDE: &str = "prelude";

pub struct Env {
    symbol_data: Vec<ObjData>,
    symbols:     Vec<Obj>,

    modules:     HashMap<String, Mod>,

    gen_sym_id: usize,
}

impl Env {
    pub fn new() -> Self {
        Self {
            symbol_data: Vec::new(),
            symbols:     Vec::new(),
            modules: hashmap! { 
                String::from(PRELUDE) => Mod::new(&String::from(PRELUDE)) 
            },
            gen_sym_id: 0
        }
    }

    pub fn add_symbol<T: Into<String>>(&mut self, symbol: T, obj: Obj) {
        let symbol = symbol.into();

        assert!(self.symbol_type(&symbol));

        self.symbol_data.push( 
            ObjData {
                is_pub:   true,
                is_const: false,
                ref_count: 0
            }
        );

        self.symbols.push(obj);

        let index = self.obj_count() - 1;
        self.module_mut(&PRELUDE.to_string())
            .unwrap()
            .add_symbol(index, &symbol);
    }

    pub fn add_symbol_to<T: Into<String>>(&mut self, module: T, symbol: T, obj: Obj) {
        let module = module.into();
        let symbol = symbol.into();

        assert!(self.symbol_type(&symbol));
        assert!(self.has_module(&module));

        self.symbol_data.push( 
            ObjData {
                is_pub:   true,
                is_const: false,
                ref_count: 0
            }
        );

        self.symbols.push(obj);

        let index = self.obj_count() - 1;
        self.module_mut(&module)
            .unwrap()
            .add_symbol(index, &symbol);    
    }

    pub fn new_module<T: Into<String>>(&mut self, name: T) {
        let name = name.into();
        assert!( !self.has_module(&name) );

        let (k, v) = Mod::new_key(&name);
        self.modules.insert(k, v);
    }

    pub fn new_module_from_file<T: Into<String>>(&mut self, name: T, path: T) {
        let name = name.into();
        assert!( !self.has_module(&name) );

        let (k, v) = Mod::new_key(&name);
        self.modules.insert(k.clone(), v);

        parse_toks(self, &k, &to_toks(&fs::read_to_string(path.into()).unwrap()));
    }

    pub fn gen_symbol_unique(&mut self) -> String {
        self.gen_sym_id += 1;
        format!("{}{}{}{}{}", "__", GEN_SYM, "-", self.gen_sym_id - 1, "__")
    }

    /// Stats and Data
    
    pub fn obj_count(&self) -> usize {
        self.symbols.len()
    }

    pub fn has_module<T: Into<String>>(&self, name: T) -> bool {
        self.modules.contains_key(&name.into())
    }

    fn symbol_type(&mut self, symbol: &String) -> bool {
        let beg = &symbol[0..2] == "__";
        let end = &symbol[symbol.len() - 2..] == "__";
        let mid = symbol[2..symbol.len() - 2].parse::<usize>();

        if beg && end && matches!(mid, Ok(n) if n == self.gen_sym_id) {
            self.gen_sym_id += 1;
            true
        } else {
            false
        }
    }

    /// Getters

    pub fn module_mut<T: Into<String>>(&mut self, name: T) -> Option<&mut Mod> {
        self.modules.get_mut(&name.into())
    }

    pub fn module<T: Into<String>>(&self, name: T) -> Option<&Mod> {
        self.modules.get(&name.into())
    }

    pub fn obj_at(&self, index: usize) -> Option<&Obj> {
        self.symbols.get(index)
    }
    
    pub fn obj_at_mut(&mut self, index: usize) -> Option<&mut Obj> {
        self.symbols.get_mut(index)
    }

    pub fn obj_mut<T: Into<String>>(&mut self, symbol: T) -> Option<&mut Obj> {
        let symbol = symbol.into();

        for module in self.modules.values() {
            if let Some(index) = module.symbol_index(self, &symbol) {
                return Some(&mut self.symbols[index]);
            }
        }
        None 
    }
}