
use crate::core::objects::Node;
use super::tokens::{Tok, Spec};

pub fn tokens_to_nodes(tokens: &Vec<Tok>) -> *mut Node {
    for spec in tokens.iter().map(|tok| &tok.spec) {
        
    }

    todo!()
}