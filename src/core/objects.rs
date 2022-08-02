
use std::ops::Deref;

use super::{
    nodes::Node,

    env::{
        Shared, 
        Env
    },
    
    types::TypeId, 

    functions::{
        FnStaticImpl,
        TupleCast, 
        FnNative, 
        FnBridge, 
        FnStatic, 
        Bridge, 
        Static, FnMacro, 
    }, 
};

#[derive(Clone)]
pub struct Quote {
    pub obj: Shared<Obj>, 
    pub evaled: bool
}

#[derive(Clone)]
pub enum Obj {
    // primitves
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(bool),

    // heap
    Str(String),

    List(Node),
    Quote(Shared<Obj>),

    // functions
    FnStatic(FnStatic),
    FnNative(FnNative),
    FnBridge(FnBridge),

    FnMacro(FnMacro),

    Nil(),
}

impl Default for Obj {
    fn default() -> Self {
        Obj::Nil()
    }
}

impl Obj {
    // TODO: make it actually const
    pub fn new_const<T: TypeId>(val: T) -> Obj {
        val.into_obj()
    }

    pub fn new_bridge(bridge: Bridge) -> Obj {
        Obj::FnBridge(FnBridge { func: bridge })
    }

    pub fn new_static<A, R>(static_fn: Static<A, R>) -> Obj 
        where A: 'static + TupleCast, R: 'static + TypeId 
    {
        let static_impl = FnStaticImpl::<A, R> { func: static_fn };
        Obj::FnStatic(FnStatic { func: Box::new(static_impl) })
    }

    pub fn set(&mut self, other: &Obj) {
        *self = other.clone();
    }

    pub fn set_to<T: TypeId>(&mut self, other: T) {
        *self = other.into_obj();
    } 

    pub fn from_string(src: &String) -> Obj {    
        if let Ok(is_i32) = src.parse::<i32>() {
            return Obj::I32(is_i32)
        }
        
        if let Ok(is_f32) = src.parse::<f32>() {
            return Obj::F32(is_f32)
        }
    
        Obj::Nil()
    }
    
    pub fn to_string(&self, env: &Env) -> String {
        match self {
            Obj::F32(x) => x.to_string(),
            Obj::F64(x) => x.to_string(),
            Obj::U32(x) => x.to_string(),
            Obj::U64(x) => x.to_string(),
            Obj::I32(x) => x.to_string(),
            Obj::I64(x) => x.to_string(),
            Obj::Str(x) => x.clone(),
            Obj::Bool(x) => x.to_string(),

            Obj::FnStatic(_) => "<static-fn>".to_string(),
            Obj::FnNative(_) => "<native-fn>".to_string(),
            Obj::FnBridge(_) => "<bridge-fn>".to_string(),
            Obj::FnMacro(_) => "<macro>".to_string(),
            
            Obj::Nil() => String::from("nil"),     

            Obj::Quote(x) => {
                if let Obj::List(_) = x.borrow().deref() {
                    x.borrow().to_string(env).to_uppercase()
                } 
                else {
                    match env.symbol_id(x) {
                        Some(s) => s.to_uppercase(),
                        None => "<UNKOWN-SYM>".to_string()
                    }
                }
            }

            Obj::List(node) => format!("({})", node.into_iter().fold(String::new(), |acc, o| {
                if acc.is_empty() {
                    format!("{}", o.to_string(env))
                } else {
                    format!("{} {}", acc, o.to_string(env))
                }
            })),
        }
    }
}