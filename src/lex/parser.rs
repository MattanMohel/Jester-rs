
use crate::core::env::Env;
use crate::core::objects::{Obj, Node};
use crate::core::modules::Mod;

use super::lexer::to_obj;
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

pub fn parse_toks(env: &mut Env, module: &String, toks: &[Tok]) -> (Node, usize) {
    let mut node = Node::new();
    let mut is_rec_end = false;
    let mut skip: usize = 0;

    for (i, tok) in toks.iter().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        match tok.spec {        
            Spec::Beg => {
                let symbol = env.gen_symbol_unique();
                let (vec, skipped) = parse_toks(env, module, &toks[i + 1..]);

                env.add_symbol_to(module, &symbol, Obj::Args(vec));     
                skip = skipped;
                
                node.args.push(env.obj_count() - 1);
                is_rec_end = true;
            },

            Spec::End => {
                if !is_rec_end || toks.get(i + 1).is_none() {
                    return (node, i);
                }
            },

            Spec::Symbol => {
                if !env.module(module).unwrap().has_symbol(env, &tok.symbol) {
                    env.add_symbol_to(module, &tok.symbol, to_obj(&tok.symbol));
                }

                node.args.push(
                    env.module(module)
                        .unwrap()
                        .symbol_index(env, &tok.symbol)
                        .unwrap());
            }
        }
    }

    panic!("unbalanced parenthesis!")
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

