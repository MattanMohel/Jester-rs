use bitflags::bitflags;
use TokType::*;

use std::sync::atomic::{
    Ordering, 
    AtomicUsize
};

const ESCAPERS:  [char; 6] = [' ', ';', '\n', '\t', '\0', '\"'];
const DELIMITER: [char; 6] = ['(', ')', '\'', ',', '@', '&']; 

#[derive(Clone)]
pub struct Tok {
    pub source: String,
    pub tok_type: TokType,  
    id: Id
}

impl Tok {
    pub fn new(source: &String, tok_type: TokType) -> Self {
        
        Self { 
            source: source.clone(), 
            tok_type, 
            id: Id::new()
        }
    } 

    pub fn new_op(tok_type: TokType) -> Self {
        Self::new(&String::new(), tok_type)
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}

#[derive(Clone, Copy)]
pub struct Id {
    id: Option<usize>
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        match (self.id, other.id) {
            (Some(a), Some(b)) => a == b,
            _ => false
        }
    }
}

impl Id {
    pub fn new() -> Self {
        static TOK_ID: AtomicUsize = AtomicUsize::new(0);
       
        Self { 
            id: Some(TOK_ID.fetch_add(1, Ordering::Relaxed))
        }
    }

    pub fn new_uninit() -> Self {
        Self { 
            id: None 
        }
    }
}


#[derive(Copy, Clone, PartialEq)]
pub enum TokType {
    Symbol,
    Beg,
    End,

    Quote,
    Escape,
    Apply,
    Rest
}

impl TokType {
    pub fn to_string(&self) -> String {
        match self {
            Symbol => "symbol".to_string(),
            Beg => "beg".to_string(),
            End => "end".to_string(),
            Quote => "quote".to_string(),
            Escape => "escape".to_string(),
            Apply => "apply".to_string(),
            Rest => "rest".to_string()
        }
    }

    pub fn is_escaper(ch: char) -> bool {
        for c in ESCAPERS.iter() {
            if ch == *c {
                return true
            }
        }
    
        false
    }
    
    pub fn is_operater(ch: char) -> bool {
        for c in DELIMITER.iter() {
            if ch == *c {
                return true
            }
        }
    
        false
    }
    
    pub fn is_non_symbol(ch: char) -> bool {
        TokType::is_escaper(ch) || TokType::is_operater(ch)
    }    
}

bitflags! {
    pub struct TokFlag: u8 {
        const ESCAPE = 0b0001;
        const QUOTE  = 0b0010;
        const APPLY  = 0b0100;
        const REST   = 0b1000;
    }
}

impl Into<TokType> for TokFlag {
    fn into(self) -> TokType {
        if self.contains(TokFlag::QUOTE) {
            Quote
        } else if self.contains(TokFlag::APPLY) {
            Apply
        } else if self.contains(TokFlag::ESCAPE) {
            Escape
        } else {
            panic!()
        }
    }
}

impl TokFlag {
    pub fn add(&mut self, tok_type: TokType) {
        match tok_type {
            Escape => *self |= TokFlag::ESCAPE,
            Quote => *self |= TokFlag::QUOTE,
            Apply => *self |= TokFlag::APPLY,
            Rest => *self |= TokFlag::REST,
            _ => ()
        }
    }

    pub fn has(&self, tok_type: TokType) -> bool {
        match tok_type {
            Escape => self.contains(TokFlag::ESCAPE),
            Quote => self.contains(TokFlag::QUOTE),
            Apply => self.contains(TokFlag::APPLY),
            Rest => self.contains(TokFlag::ESCAPE),
            _ => panic!()
        }
    }
}

#[derive(Clone)]
pub struct Expr {
    pub tok_beg: Id,
    pub tok_end: Id,
    pub flag: TokFlag,
    pub exprs: Vec<Expr>,
}

impl Expr {
    pub fn new(tok_beg: &Id, flag: TokFlag) -> Self {
        Self::from_parts(tok_beg, &Id::new_uninit(), flag, &Vec::new())
    }

    pub fn from_parts(tok_beg: &Id, tok_end: &Id, flag: TokFlag, exprs: &Vec<Expr>) -> Self {
        Expr { 
            tok_beg: *tok_beg, 
            tok_end: *tok_end, 
            flag: flag,
            exprs: exprs.clone()
        }
    }

    pub fn for_each<F>(&self, f: &mut F)
        where F: FnMut(&Expr) {
        
        f(self);

        for expr in self.exprs.iter() {
            expr.for_each(f);
        }
    }

    pub fn for_each_chosen<F>(&self, f: &mut F)
        where F: FnMut(&Expr) -> bool {
        
        if !f(self) {
            return
        }

        for expr in self.exprs.iter() {
            expr.for_each_chosen(f);
        }
    }

    pub fn any<F>(&self, f: &mut F) -> bool
        where F: FnMut(&Expr) -> bool {
        
        if f(self) {
            return true
        }

        for expr in self.exprs.iter() {
            if expr.any(f) {
                return true
            }
        }

        false
    }
}