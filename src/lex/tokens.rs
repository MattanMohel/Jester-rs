
#[derive(Clone)]
pub enum Spec {
    Symbol,

    ListBeg,
    ListEnd,
}

#[derive(Clone)]
pub struct Tok {
    pub symbol: String,
    pub spec:   Spec,
    
    pub line: usize,
}

impl Tok {
    pub fn new(symbol: &String, spec: &Spec, line: usize) -> Tok {
        Tok { symbol: symbol.clone(), spec: spec.clone(), line: line }
    }
}