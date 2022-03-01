
use std::collections::HashMap;

use super::objects::{Obj, Node};
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
    // symbol-value module bindings
    symbols: HashMap<String, Obj>,
    // all imported modules
    imports: Vec<Module>,
    // tokenized source of the module
    tokens: Vec<Tok>,
    // string source of the module
    filesrc: String,
    // module's internal alias
    mod_symbol: String,
    // pointer to the beginning
    // node of the module
    stack_ptr: *mut Node,
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
            stack_ptr: std::ptr::null_mut(), //temporary
        }
    }

    pub fn debug(&self) {
        for tok in self.tokens.iter() {
            println!("{}", tok.symbol);
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