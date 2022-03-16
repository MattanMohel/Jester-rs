
use super::objects::{Obj, ObjData};
use super::modules::Mod;

const GEN_SYM: &str = "$gensym";

pub struct Env {
    symbol_data: Vec<ObjData>,
    symbols:     Vec<Obj>,
    modules:     Vec<Mod>,

    curr_id: usize
}

impl Env {
    pub fn new() -> Self {
        Self {
            symbol_data: Vec::new(),
            symbols:     Vec::new(),
            modules: vec![Mod::new("prelude")],
            curr_id: 0
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

        let index = self.obj_index();
        self.modules[0].add_symbol(index, &symbol);
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
        let index = self.obj_index();
        self.get_mod_mut(&module).unwrap().add_symbol(index, &symbol);
    }

    pub fn gen_symbol_unique(&mut self) -> String {
        let symbol = format!("{}{}{}{}{}", "__", GEN_SYM, "-", self.curr_id, "__");
        self.curr_id += 1;

        symbol
    }

    /// Statistics
    
    pub fn obj_index(&self) -> usize {
        self.symbols.len() - 1
    }

    pub fn has_mod<T: Into<String>>(&self, symbol: T) -> bool {
        let symbol = symbol.into();
        for module in self.modules.iter() {
            if *module.name() == symbol {
                return true;
            }
        }
        false
    }

    fn is_allowed_symbol(symbol: &String) -> bool {   
        if let Some(_) = symbol.find(GEN_SYM)  {
            return false;
        }
        true
    }

    /// Getters

    pub fn get_mod_at(&self, index: usize) -> &Mod {
        &self.modules[index]
    }

    pub fn get_mod_at_mut(&mut self, index: usize) -> &mut Mod {
        &mut self.modules[index]
    }

    pub fn get_mod_mut<T: Into<String>>(&mut self, symbol: T) -> Option<&mut Mod> {
        let symbol = symbol.into();
        for module in self.modules.iter_mut() {
            if *module.name() == symbol {
                return Some(module);
            }
        }
        None
    }

    pub fn get_obj_at(&self, index: usize) -> &Obj {
        &self.symbols[index]
    }
    
    pub fn get_obj_at_mut(&mut self, index: usize) -> &mut Obj {
        &mut self.symbols[index]
    }

    pub fn get_obj_mut<T: Into<String>>(&mut self, symbol: T) -> Option<&mut Obj> {
        let symbol = symbol.into();
        for module in self.modules.iter() {
            if let Some(index) = module.symbol_index(self, &symbol) {
                return Some(&mut self.symbols[index]);
            }
        }
        None 
    }
}