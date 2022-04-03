use crate::core::{
    env::Env,
    objects::Obj,
    nodes::Node, 

    err::{
        JtsErrType::*, 
        AsResult, 
        JtsErr
    },
};

use super::{
    lexer::{
        to_obj, 
        to_toks
    },

    tokens::{
        Tok, 
        Spec
    },
};

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

pub fn parse_file(env: &mut Env, mod_id: &String, path: &String) -> JtsErr<Node> {
    let src = std::fs::read_to_string(path)?;   
    let toks = to_toks(&src);

    parse_toks(env, mod_id, &toks)
}

pub fn parse_src(env: &mut Env, mod_id: &String, src: &String) -> JtsErr<Node> {
    let toks = to_toks(&src);

    parse_toks(env, mod_id, &toks)
}

fn parse_toks(env: &mut Env, mod_id: &String, toks: &Vec<Tok>) -> JtsErr<Node> {
    let mut node_curr  = Node::default();
    let mut nodes_prev = Vec::new();
    let mut parenth: isize = 0;
    
    for tok in toks.iter() {
        match tok.spec {        
            Spec::Beg => {
                parenth += 1;

                nodes_prev.push(node_curr);
                node_curr = Node::default();
            },

            Spec::End => {
                parenth -= 1;

                match nodes_prev.pop() {
                    Some(mut node_prev) => {
                        let symbol = env.unique_symbol();  

                        env.add_symbol_to(mod_id, &symbol, Obj::Node(node_curr))?;
                        node_prev.args.push(env.symbol(&symbol)?);

                        node_curr = node_prev;
                    },

                    None => break,
                }
            },

            Spec::Symbol => {
                if !env.module(mod_id).unwrap().borrow_mut().symbol(&tok.symbol).is_some() {
                    env.add_symbol_to(mod_id, &tok.symbol, to_obj(&tok.symbol))?;
                }

                node_curr.args.push(env.symbol(&tok.symbol).unwrap());
            }
        }
    }

    // env.module(mod_id)?.borrow_mut().add_body(node_curr);

    (parenth == 0).as_result(node_curr, UnbalancedParentheses)
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

