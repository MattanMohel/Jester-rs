
use maplit::hashmap;

use super::objects::{Obj, ObjData};
use super::modules::Mod;

use std::collections::HashMap;

const GEN_SYM: &str = "$gensym";
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

        assert!(Env::is_allowed_symbol(&symbol));

        self.symbol_data.push( 
            ObjData {
                is_pub:    true,
                is_const:  false,
                module:    0,
                ref_count: 0
            }
        );

        self.symbols.push(obj);

        let index = self.obj_count();
        self.module_mut(PRELUDE).unwrap().add_symbol(index, &symbol);
    }

    pub fn add_symbol_to<T: Into<String>>(&mut self, module: T, symbol: T, obj: Obj) {
        let module = module.into();
        let symbol = symbol.into();

        assert!(Env::is_allowed_symbol(&symbol));

        self.symbol_data.push( 
            ObjData {
                is_pub:    true,
                is_const:  false,
                module:    0,
                ref_count: 0
            }
        );

        self.symbols.push(obj);

        assert!(self.has_mod(&module));
        let index = self.obj_count();
        self.module_mut(&module).unwrap().add_symbol(index, &symbol);
    }

    pub fn gen_symbol_unique(&mut self) -> String {
        self.gen_sym_id += 1;
        format!("{}{}{}{}{}", "__", GEN_SYM, "-", self.gen_sym_id - 1, "__")
    }

    /// Stats and Data
    
    pub fn obj_count(&self) -> usize {
        self.symbols.len() - 1
    }

    pub fn has_mod<T: Into<String>>(&self, name: T) -> bool {
        self.modules.contains_key(&name.into())
    }

    fn is_allowed_symbol(symbol: &String) -> bool {   
        symbol.find(GEN_SYM).is_none()
    }

    /// Getters

    pub fn module_mut<T: Into<String>>(&mut self, name: T) -> Option<&mut Mod> {
        self.modules.get_mut(&name.into())
    }

    pub fn module<T: Into<String>>(&self, name: T) -> Option<&Mod> {
        self.modules.get(&name.into())
    }

    pub fn obj_at(&self, index: usize) -> &Obj {
        &self.symbols[index]
    }
    
    pub fn obj_at_mut(&mut self, index: usize) -> &mut Obj {
        &mut self.symbols[index]
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