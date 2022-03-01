
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

fn is_non_token(ch: char) -> bool {
    is_escaper(ch) || is_operater(ch)
}

pub fn read_file(filepath: &String) -> String {

    println!("filepath: {}", filepath);

    if let Ok(src) = fs::read_to_string(filepath) {
        return src
    }

    panic!("filepath doesn't exist")
}

fn str_to_spec(src: &String) -> Option<Spec> {
    assert!( !src.is_empty() );
    
    if let Ok(is_i32) = src.parse::<i32>() {
        return Some(Spec::Value(Type::I32(is_i32)))
    }
    
    if let Ok(is_f32) = src.parse::<f32>() {
        return Some(Spec::Value(Type::F32(is_f32)))
    }

    None
}

pub fn str_to_token(src: &String, line: usize) -> Option<Tok>{
    if src.is_empty() || is_escaper(src.chars().nth(0).unwrap()) {
        return None
    }

    let spec = if src == "(" {
        Spec::ListBeg
    } else 
    if src == ")" {
        Spec::ListEnd
    } else 
    if let Some(val) = str_to_spec(src) {
        val
    } else {
        Spec::Symbol
    };

    Some(Tok::new(src, &spec, line))
}

pub fn file_to_tokens(src: &String) -> Vec<Tok> {

    let mut tokens = Vec::new();
    let mut is_string = false;
    
    let mut parenth_depth = 0isize;
    let mut line  = 0usize;
  
    let mut lex = String::new();

    for ch in src.chars() {
        if is_string || !is_non_token(ch) {
            lex.push(ch);
            continue;
        }

        if let Some(tok) = str_to_token(&lex, line) {
            tokens.push(tok);
            lex.clear();
        }

        match ch {
            '(' => parenth_depth += 1,
            ')' => parenth_depth -= 1,

            '\"' => is_string = !is_string,
            
            '\n' => line += 1,
            
            _ => ()
        }

        if let Some(tok) = str_to_token(&ch.to_string(), line) {
            tokens.push(tok);
        }
    }

    assert!(parenth_depth == 0);

    tokens
}