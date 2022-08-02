use TokType::*;

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

use super::tokens::{
    Id,
    Tok, 
    TokType, 
    Expr,
    TokFlag, 
};

pub struct Parser {
    tokens: Vec<Tok>,
    exprs: Vec<Expr>
}

impl Parser {
    pub fn from_file(path: &String) -> JtsErr<Self> {
        let src = std::fs::read_to_string(path)?;
        Parser::from_string(&src)
    }
    
    pub fn from_string(src: &String) -> JtsErr<Self> {
        let tokens = Parser::extract_tokens(&src);
        let exprs = Parser::extract_exprs(&tokens);

        Ok(
            Self {
                tokens: tokens,
                exprs: exprs
        })
    } 

    fn index_at_expr(&self, expr: &Expr) -> (usize, usize) {
        let beg = self.tokens.iter().position(|tok| *tok.id() == expr.tok_beg).unwrap();
        let end = self.tokens.iter().position(|tok| *tok.id() == expr.tok_end).unwrap();

        (beg, end)
    }

    fn expr_at_tok(&self, tok_beg: &Id) -> Option<&Expr> {
        for expr in self.exprs.iter() {
            if expr.tok_beg == *tok_beg {
                return Some(expr)
            }
        }
        None
    }

    fn parent_expr(&self, expr: &Expr) -> &Expr {
        let (beg, _) = self.index_at_expr(expr);

        let iter = self.tokens.iter()
            .rev()
            .skip(self.tokens.len() - beg);
        
        for tok in iter {
            if tok.tok_type == Beg {
                return self.expr_at_tok(tok.id()).unwrap()
            }
        }

        panic!("expression has no parent")
    }

    fn extract_tokens(src: &String) -> Vec<Tok> {
        
        // converts a lexical buffer into a matching token
        // and returns None if the buffer is empty 
        fn tok_from_string(src: &String) -> Option<Tok>{
            if src.is_empty() || TokType::is_escaper(src.chars().nth(0).unwrap()) {
                return None
            }
        
            Some(
                match src.as_str() {
                    "(" => Tok::new_op(Beg),
                    ")" => Tok::new_op(End),
                    "\'" => Tok::new_op(Quote),
                    "," => Tok::new_op(Escape),
                    "@" => Tok::new_op(Apply),
                    "&" => Tok::new_op(Rest),
                    _ => Tok::new(src, TokType::Symbol)
            })
        }

        // buffer of tokens
        let mut toks = Vec::new();    
        // line count
        let mut _line: usize  = 0;
        // parenthesis depth
        let mut depth: isize = 0;
        // is string literal
        let mut str = false;
        // lexed buffer
        let mut lex = String::new();
    
        for ch in src.chars() {
            if str || !TokType::is_non_symbol(ch) {
                lex.push(ch);
                continue;
            }
    
            if let Some(tok) = tok_from_string(&lex) {
                toks.push(tok);
                lex.clear();
            }

            match ch {
                '(' => depth += 1,
                ')' => depth -= 1,
                '\"' => str = !str,    
                '\n' => _line += 1,
                
                _ => ()
            }
    
            if let Some(tok) = tok_from_string(&ch.to_string()) {
                toks.push(tok);
            }
        }
    
        if let Some(tok) = tok_from_string(&lex) {
            toks.push(tok);
        }
    
        assert!(depth == 0);
    
        toks
    }

    fn extract_exprs (toks: &[Tok]) -> Vec<Expr> {
        let mut exprs = Vec::new();
        let mut flag = TokFlag::empty();

        for tok in toks.iter() {
            match tok.tok_type {
                Beg => {
                    exprs.push(Expr::new(tok.id(), flag));
                    
                    flag = TokFlag::empty();
                },
                
                End => {
                    let mut expr = exprs.pop().unwrap();
                    expr.tok_end = *tok.id();

                    if exprs.is_empty() {
                        exprs.push(expr);
                    } else {
                        exprs.last_mut().unwrap().exprs.push(expr);
                    }
                },
     
                Symbol => {
                    let expr = Expr::from_parts(tok.id(), tok.id(), flag, &Vec::new());
                    
                    if exprs.is_empty() {
                        exprs.push(expr);
                    } else {
                        exprs.last_mut().unwrap().exprs.push(expr);
                    }
                    
                    flag = TokFlag::empty();
                }
                
                Escape | Quote | Apply | Rest => flag.add(tok.tok_type),
            }
        }

        exprs
    }

    fn extract_opers(&mut self) { 
        let exprs = self.exprs.clone();

        for expr in exprs.iter() {
            expr.for_each(&mut |e1| {

                // Apply
                if e1.flag.has(Apply) {
                    self.map_expr(e1, Apply)
                }

                if e1.flag.has(Quote) {
                    e1.for_each_chosen(&mut |e2| {
                        if e2.flag.has(Escape) {
                            false
                        } else {
                            if !e2.any(&mut |e3| e3.flag.has(Escape) ) {
                                self.map_expr(e2, Quote);
                                return false
                            } 

                            true
                        }
                    })
                }
            })
        }

        self.tokens.retain(|tok| {
            match tok.tok_type {
                Quote | Escape | Apply | Rest => false,
                _ => true
            }
        })
    }

    fn map_expr(&mut self, expr: &Expr, op: TokType) {

        let (beg, end) = self.index_at_expr(expr);

        match op {
            Escape => {
                self.tokens.insert(beg, Tok::new_op(Escape));
            }
            Quote => {
                self.tokens.insert(end + 1, Tok::new_op(End));
                self.tokens.insert(beg, Tok::new(&op.to_string(), Symbol));
                self.tokens.insert(beg, Tok::new_op(Beg));
                
            }
            Apply => {
                let parent = self.parent_expr(expr);
                let (parent_beg, _) = self.index_at_expr(parent);

                self.tokens.insert(parent_beg + 1, Tok::new(&op.to_string(), Symbol))
            }
            _ => ()
        }
    }

    /// parses code into recursive linked lists of Objects
    ///
    /// for effeciency the linked lists are represented by vectors
    ///
    /// ```
    /// 1: (set x (+ 5 5))
    /// 2: (+= x 10)   
    /// 3: 
    /// 4: x 
    /// ```
    /// translates to...
    ///
    /// ```
    /// (...) --> (...) --> 'x'
    ///   \         \__ '+=' --> 'x' --> '10'
    ///    \ 
    ///     \__ 'set' --> 'x' --> (...)
    ///                             \__ '+' --> '5' --> '5'
    /// ```
    pub fn parse_tokens (&mut self, env: &mut Env, mod_id: &String) -> JtsErr<Node> {
        self.extract_opers();

        let mut node_curr  = Node::default();
        let mut nodes_prev = Vec::new();
        let mut parenth: isize = 0;
            
        for tok in self.tokens.iter() {
            match tok.tok_type {        
                Beg => {
                    parenth += 1;

                    nodes_prev.push(node_curr);
                    node_curr = Node::default();
                },

                End => {
                    parenth -= 1;

                    if let Some(mut node_prev) = nodes_prev.pop() {
                        let unique_symbol = env.unique_symbol();  

                        env.add_symbol_to(mod_id, &unique_symbol, Obj::List(node_curr))?;
                        node_prev.args.push(env.symbol(&unique_symbol)?);

                        node_curr = node_prev;
                    }
                },

                Symbol => {
                    let symbol = env.module(mod_id).unwrap()
                        .borrow_mut()
                        .symbol(&tok.source);

                    if !symbol.is_some() {
                        env.add_symbol_to(mod_id, &tok.source, Obj::from_string(&tok.source))?;
                    }

                    node_curr.args.push(env.symbol(&tok.source).unwrap());
                }

                _ => unreachable!()
            }
        }

        // env.module(mod_id)?.borrow_mut().add_body(node_curr);

        (parenth == 0).as_result(node_curr, UnbalancedParentheses)
    }
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

