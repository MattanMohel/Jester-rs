
use super::objects::Obj;
use super::modules::Module;

pub struct ObjData {
    pub(in super::env) is_pub:    bool,
    pub(in super::env) is_const:  bool,
    pub(in super::env) module:    usize,
    pub ref_count: usize,
}

impl ObjData {
    pub fn is_pub(&self) -> bool {
        self.is_pub
    }

    pub fn is_const(&self) -> bool {
        self.is_const
    }
}

pub struct Env {
    symbol_data: Vec<ObjData>,
    symbols:     Vec<Obj>,
    modules:     Vec<Module>,

    curr_id: usize
}

impl Env {
    pub fn new() -> Self {
        Self {
            symbol_data: Vec::new(),
            symbols:     Vec::new(),
            modules: vec![Module::new("prelude")],
            curr_id: 0
        }
    }

    // generates a garaunteed unique variable symbol
    // follows the form: '__gensym-###___

    pub fn gensym_unique(&mut self) -> String {
        const PREFIX:  &str = "__gensym-";
        const POSTFIX: &str = "__";
        
        let symbol = format!("{}{}{}", PREFIX, self.curr_id, POSTFIX);
        self.curr_id += 1;

        symbol
    }

    // checks if a given symbol is allowed by the env
    // a symbol is disallowed if it conflicts with gensym (see 'gensym_unique')

    fn is_disallowed_symbol(symbol: &String) -> bool {
        const PREFIX:  &str = "__gensym-";
        const POSTFIX: &str = "__";
        
        let mut index = -1_isize;        
        for (i, c) in (&symbol[8..]).chars().enumerate() {
            if !c.is_numeric() {
                index = i as isize;
            }
        }

        let beg = PREFIX == &symbol[0..PREFIX.len()];
        let end = index != -1 && POSTFIX == &symbol[index as usize..];
        
        beg && end
    }

    // adds a given symbol to the default 'prelude' module
    // such a symbol, by default, is const and public

    pub fn add_symbol(&mut self, symbol: &str, obj: Obj) -> usize {
        let symbol = symbol.to_string();

        assert!( !Env::is_disallowed_symbol(&symbol) );

        self.symbols.push(obj);

        let data =  ObjData { 
            is_pub:    true, 
            is_const:  true, 
            module:    0,
            ref_count: 0,
        };
        self.symbol_data.push(data);

        let index = self.symbols.len() - 1;
        self.modules[0].add_symbol(&symbol, index);

        index
    }

    // adds a given symbol to a given module
    // such a symbol, by default, is const and public

    pub fn add_symbol_to_module(&mut self, module: &mut Module, symbol: &str, obj: Obj) -> usize {
        let symbol = symbol.to_string();

        assert!( !Env::is_disallowed_symbol(&symbol) );

        self.symbols.push(obj);

        let data =  ObjData { 
            is_pub:    true, 
            is_const:  true, 
            module:    0,
            ref_count: 0,
        };
        self.symbol_data.push(data);

        let index = self.symbols.len() - 1;
        module.add_symbol(&symbol, index);

        index
    }

    // returns module at the specified index

    pub fn get_mod_at(&self, index: usize) -> &Module {
        &self.modules[index]
    }

    // returns mut module at the specified index

    pub fn get_mod_at_mut(&mut self, index: usize) -> &mut Module {
        &mut self.modules[index]
    }

    // returns mut module by specified name

    pub fn get_mod_mut(&mut self, module_name: &String) -> Option<&mut Module> {
        for module in self.modules.iter_mut() {
            if module.name() == module_name {
                return Some(module)
            }
        }

        None
    }

    // returns object at the specified index

    pub fn get_obj_at(&self, index: usize) -> &Obj {
        &self.symbols[index]
    }

    // returns mut object at the specified index
    
    pub fn get_obj_at_mut(&mut self, index: usize) -> &mut Obj {
        &mut self.symbols[index]
    }

    // returns mut object by specified name

    pub fn get_obj_mut(&mut self, symbol: &String) -> Option<&mut Obj> {
        for module in self.modules.iter() {
            if let Some(index) = module.get_symbol_index(symbol) {
                return Some(&mut self.symbols[index])
            }
        }

        None 
    }
}