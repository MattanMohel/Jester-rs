
use crate::core::types::Type;

pub struct Tok {
    symbol: String,
    var: Type,
    
    line: usize,
}