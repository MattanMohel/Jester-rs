
use crate::core::objects::Node;
use super::tokens::{Tok, Spec};



/*

parses code into recursive linked lists of Objects

________________________

1: (set x (+ 5 5))
2: (+= x 10)   
3: 
4: x 
________________________

translates to...
________________________

(...) --> (...) --> 'x'
  \         \__ '+=' --> 'x' --> '10'
   \ 
    \__ 'set' --> 'x' --> (...)
                            \__ '+' --> '5' --> '5'
________________________

this form can be easily traversed and evaluated

*/

pub fn tokens_to_nodes(tokens: &Vec<Tok>) -> *mut Node {
    for tok in tokens.iter() {
        match &tok.spec {
            Spec::ListBeg => {
                
            },

            Spec::ListEnd => {

            },

            Spec::Symbol => {

            },

            Spec::Value(val) => {

            },
        }
    }

    todo!()
}