
use super:: {
    nodes::Node,
    env::Shared,
    types::TypeId, 

    functions::{
        FnNative, 
        FnBridge, Bridge
    }, 
};

use std::fmt;

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

    // heaps
    Str(String),

    // functions
    FnRust(),
    FnNative(FnNative),
    FnBridge(FnBridge),

    Node(Node),
    Lazy(Shared<Obj>),

    Nil(),
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Default for Obj {
    fn default() -> Self {
        Obj::Nil()
    }
}

impl Obj {
    pub fn new_const<T: TypeId>(val: T) -> Obj {
        val.into_obj()
    }

    pub fn new_bridge(bridge: Bridge) -> Obj {
        Obj::FnBridge(FnBridge { func: bridge })
    }

    pub fn set(&mut self, other: &Obj) {
        *self = other.clone();
    }

    pub fn set_to<T: TypeId>(&mut self, other: T) {
        *self = other.into_obj();
    } 
    
    pub fn to_string(&self) -> String {
        match self {
            Obj::F32(x) => x.to_string(),
            Obj::F64(x) => x.to_string(),
            Obj::U32(x) => x.to_string(),
            Obj::U64(x) => x.to_string(),
            Obj::I32(x) => x.to_string(),
            Obj::I64(x) => x.to_string(),
            Obj::Str(x) => x.clone(),
            Obj::Bool(x) => x.to_string(),

            Obj::Lazy(x) => x.borrow().to_string(),

            Obj::Node(node) => format!("({})", node.into_iter()
                .fold(String::new(), |acc, o| {
                    if acc.is_empty() {
                        format!("{}", o.to_string())
                    } else {
                        format!("{} {}", acc, o.to_string())
                    }})),

            Obj::FnRust() =>    "<rust>".to_string(),
            Obj::FnNative(_) => "<native>".to_string(),
            Obj::FnBridge(_) => "<bridge>".to_string(),
            Obj::Nil() => String::from("nil"),     
        }
    }
}