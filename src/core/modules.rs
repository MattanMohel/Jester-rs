
use std::collections::HashMap;

use super::objects::{Obj, Node};
use crate::lex::tokens::Tok;

use crate::lex::lexer::{read_file, file_to_tokens};
use crate::lex::parser::tokens_to_nodes;

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
    src: String,

    // module's internal alias
    name: String,

    stack_ptr: *mut Node,
}

impl Module {
    pub fn new(root: &String, name: &String) -> Module {     
        let mut module = Module {
            symbols: HashMap::new(),     
            imports: Vec::new(),
            tokens:  Vec::new(),  
            src:  String::new(),
            name:  name.clone(),
            stack_ptr: std::ptr::null_mut(),
        };

        module.src       = read_file(&format!("{}\\{}", root, name));
        module.tokens    = file_to_tokens(&module.src);
        module.stack_ptr = tokens_to_nodes(&module.tokens);

        module
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