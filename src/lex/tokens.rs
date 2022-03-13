
#[derive(Copy, Clone)]
pub enum Spec {
    Symbol,
    // begining of list
    Beg,
    // ending of list
    End,
}

#[derive(Clone)]
pub struct Tok {
    pub symbol: String,
    pub spec:   Spec,
    
    pub line: usize,
}

impl Tok {
    pub fn new(symbol: &String, spec: Spec, line: usize) -> Tok {
        Tok { symbol: symbol.clone(), spec: spec, line: line }
    }
}