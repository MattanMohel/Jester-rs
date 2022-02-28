
use std::collections::HashMap;
use std::path::Path;

use super::objects::Obj;
use crate::lex::tokens::Tok;

use crate::lex::lexer::{read_file, file_to_tokens};

pub struct ObjData {
    symbol: String,
    val: Obj,

    is_pub:   bool,
    is_init:  bool,
    is_const: bool,
}

pub struct Module {
    // symbol - value bindings
    symbols: HashMap<String, Obj>,
    // all other imported modules
    imports: Vec<Module>,
    // tokenized source of the module
    tokens: Vec<Tok>,
    // string source of the module
    filesrc: String,
    // mod internal alias
    mod_symbol: String,
}

impl Module {
    pub fn new(root: &String, name: &String) -> Module {     
        let filepath = format!("{}\\{}", root, name);
        let filesrc  = read_file(&filepath);

        Module {
            symbols: HashMap::new(),
            imports: Vec::new(),
            tokens:  file_to_tokens(&filesrc),       
            filesrc: read_file(&filepath),
            mod_symbol: name.clone(),
        }
    }

    pub fn has(&self, symbol: &String) -> bool {
        self.symbols.contains_key(symbol)
    }
    
    pub fn add(&mut self, symbol: &String, obj: Obj) -> bool {
        if let Some(_) = self.symbols.insert(symbol.clone(), obj) {
            return false
        }
        
        true
    }
    
    pub fn get(&mut self, symbol: &String) -> Option<&mut Obj> {
        self.symbols.get_mut(symbol)
    }
}