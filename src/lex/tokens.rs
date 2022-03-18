
use std::fmt;

#[derive(Copy, Clone)]
pub enum Spec {
    Symbol,
    Beg,
    End,
}

#[derive(Clone)]
pub struct Tok {
    pub symbol: String,
    pub spec:   Spec,
    
    pub line: usize,
}

impl fmt::Display for Tok {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

impl Tok {
    pub fn new(symbol: &String, spec: Spec, line: usize) -> Tok {
        Tok { symbol: symbol.clone(), spec: spec, line: line }
    }
}