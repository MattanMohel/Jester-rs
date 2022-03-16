
use std::fs;

use super::tokens::{Tok, Spec};
use crate::core::types::Type;

const ESCAPERS:  [char; 6] = [' ', ';', '\n', '\t', '\0', '\"'];
const OPERATORS: [char; 4] = ['(', ')', '\'', '~']; 

fn is_escaper(ch: char) -> bool {
    for c in ESCAPERS {
        if ch == c {
            return true
        }
    }

    false
}

fn is_operater(ch: char) -> bool {
    for c in OPERATORS {
        if ch == c {
            return true
        }
    }

    false
}

fn is_non_tok(ch: char) -> bool {
    is_escaper(ch) || is_operater(ch)
}

pub fn to_type(src: &String) -> Type {    
    if let Ok(is_i32) = src.parse::<i32>() {
        return Type::I32(is_i32)
    }
    
    if let Ok(is_f32) = src.parse::<f32>() {
        return Type::F32(is_f32)
    }

    Type::Nil()
}

pub fn to_tok(src: &String, line: usize) -> Option<Tok>{
    if src.is_empty() || is_escaper(src.chars().nth(0).unwrap()) {
        return None
    }

    let spec = if src == "(" {
        Spec::Beg
    } else if src == ")" {
        Spec::End
    } else {
        Spec::Symbol
    };

    Some(Tok::new(src, spec, line))
}

pub fn to_toks(src: &String) -> Vec<Tok> {

    let mut toks = Vec::new();
    
    // line count
    let mut line  = 0usize;
    // parenthesis depth
    let mut depth = 0isize;
    // in string literal
    let mut str = false;

    // lexed buffer
    let mut lex = String::new();

    for ch in src.chars() {
        if str || !is_non_tok(ch) {
            lex.push(ch);
            continue;
        }

        if let Some(tok) = to_tok(&lex, line) {
            toks.push(tok);
            lex.clear();
        }

        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,

            '\"' => str = !str,
            
            '\n' => line += 1,
            
            _ => ()
        }

        if let Some(tok) = to_tok(&ch.to_string(), line) {
            toks.push(tok);
        }
    }

    assert!(depth == 0);

    toks
}