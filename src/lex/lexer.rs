
use std::fs;

use super::tokens::Tok;

pub fn read_file(filepath: &String) -> String {
    if let Ok(src) = fs::read_to_string(filepath) {
        return src
    }

    panic!("filepath doesn't exist")
}

pub fn file_to_tokens(src: &String) -> Vec<Tok> {
    todo!()
}


const ESCAPERS:  [char; 6] = [' ', ';', '\n', '\t', '\0', '\"'];
const OPERATORS: [char; 4] = ['(', ')', '\'', '~']; 

pub fn is_escaper(ch: char) -> bool {
    for c in ESCAPERS {
        if ch == c {
            return true
        }
    }

    false
}

pub fn is_operater(ch: char) -> bool {
    for c in OPERATORS {
        if ch == c {
            return true
        }
    }

    false
}

pub fn is_non_token(ch: char) -> bool {
    is_escaper(ch) || is_operater(ch)
}

// pub fn string_type(src: &String) -> Option<Obj> {

//     assert!( !src.is_empty() );
    
//     if let Ok(is_i32) = src.parse::<i32>() {
//         return Some(Obj::I32(is_i32))
//     }
    
//     if let Ok(is_f32) = src.parse::<f32>() {
//         return Some(Obj::F32(is_f32))
//     }

//     // collect the first and last characters
//     let frst = src.chars().nth(0).unwrap();
//     let last = src.chars().last().unwrap();

//     if frst == '\"' && last == '\"' {
//         if src.len() == 3 {
//             //return Obj::Char
//         }
//         else {
//             //return Obj::String
//         }
//     }


//     None
// }

// pub fn to_token(src: &String, line: usize) -> Option<Tok>{
//     if src.is_empty() || is_escaper(src.chars().nth(0).unwrap()) {
//         return None
//     }

//     match src.as_str() {
//         "(" => Some(Tok::ListBeg),
//         ")" => Some(Tok::ListEnd),

//         "'" => Some(Tok::Quote),
//         "~" => Some(Tok::Eval ), 

//         _ => {          
//             if let Some(obj) =  string_type(&src) {
//                 return Some(Tok::Value(src.clone(), obj))
//             }  
        
//             Some(Tok::Symbol(src.clone()))
//         }
//     }
// }

// pub fn tokenize_src(src: &String) -> Vec<Tok> {

//     let mut tokens = Vec::new();
//     let mut iter = StrIter::new(src, 0);

//     let mut is_string = false;
    
//     let mut parenth_depth = 0isize;
//     let mut line  = 0usize;
  
//     let mut buffer = String::new();

//     loop {
//         while iter.exists() && ( is_string || !is_non_token(iter.elem()) ) {
//             buffer.push(iter.elem());
//             iter.advance(1);
//         }

//         if let Some(tok) = to_token(&buffer, line) {
//             tokens.push(tok);
//             buffer.clear();
//         }

//         if !iter.exists() {
//             break;
//         }

//         match iter.elem() {
//             '(' => parenth_depth += 1,
//             ')' => parenth_depth -= 1,

//             '\"' => is_string = !is_string,
            
//             '\n' => line += 1,
            
//             '\'' => (), //quote
//             '~'  => (), //eval
            
//             _ => ()
//         }

//         if let Some(tok) = to_token(&iter.elem().to_string(), line) {
//             tokens.push(tok);
//         }

//         iter.advance(1);
//     }

//     assert!(parenth_depth == 0);

//     tokens
// }