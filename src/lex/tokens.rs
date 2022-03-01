
use crate::core::types::Type;

#[derive(Clone)]
pub enum Spec {
    Symbol,
    Value(Type),

    ListBeg,
    ListEnd,
}

pub struct Tok {
    pub symbol: String,
    pub spec: Spec,
    
    pub line: usize,
}

impl Tok {
    pub fn new(symbol: &String, spec: &Spec, line: usize) -> Tok {
        Tok { symbol: symbol.clone(), spec: spec.clone(), line: line }
    }
}