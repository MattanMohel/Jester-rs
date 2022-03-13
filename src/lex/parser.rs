
use crate::core::env::Env;
use crate::core::objects::{Obj, Node};
use crate::core::modules::Module;
use crate::core::types::Type;

use super::lexer::str_to_typ;
use super::tokens::{Tok, Spec};

/*

parses code into recursive linked lists of Objects

for effeciency the linked lists are represented by vectors

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

pub fn parse_toks(env: &mut Env, module: &mut Module, toks: &[Tok]) -> Node {
    let mut node = Vec::new();

    for (i, tok) in toks.iter().enumerate() {
        match tok.spec {
            
            Spec::Beg => {
                let symbol = env.gensym_unique();
                let new_node = parse_toks(env, module, &toks[i..]);

                let obj_index = env.add_symbol(
                    symbol.as_str(), 
                    Obj::new(Type::Node(new_node)));

                node.push(obj_index);
            },

            Spec::End => break,

            Spec::Symbol => {
                if !module.has_symbol(env, &tok.symbol) {
                    env.add_symbol_to(module, 
                        &tok.symbol.as_str(), 
                        Obj::new(str_to_typ(&tok.symbol)));
                }
            }
        }
    }

    node
}

/*

expands macro forms at compile time

________________________

1: (import module-1)
2: (println "best var {}" module-1-var)
________________________

the 'import' command has a compile-time effect
as it needs to modify the state of the external module

upon reaching a macro and parsing its full body, 'expand_macro'
is called to establish its behaviour and expand it in place

*/

