
use super:: {
    nodes::Node,
    types::TypeId, 

    functions::{
        FnNative, 
        FnBridge, Bridge
    }, 
    
    err::{
        JtsErr, 
        ErrType::*
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

    // heaps
    Str(String),

    // functions
    FnRust(),
    FnNative(FnNative),
    FnBridge(FnBridge),

    Node(Node),
    Ref(usize),

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
    pub fn is_u32(&self) -> JtsErr<&u32> {
        match self {
            Obj::U32(x) => Ok(x),
            _ => Err(MismatchedType)
        }
    }

    pub fn is_u64(&self) -> JtsErr<&u64> {
        match self {
            Obj::U64(x) => Ok(x),
            _ => Err(MismatchedType)
        }
    }

    pub fn is_i32(&self) -> JtsErr<&i32> {
        match self {
            Obj::I32(x) => Ok(x),
            _ => Err(MismatchedType)
        }
    }

    pub fn is_i64(&self) -> JtsErr<&i64> {
        match self {
            Obj::I64(x) => Ok(x),
            _ => Err(MismatchedType)
        }
    }

    pub fn is_f32(&self) -> JtsErr<&f32> {
        match self {
            Obj::F32(x) => Ok(x),
            _ => Err(MismatchedType)
        }
    }

    pub fn is_f64(&self) -> JtsErr<&f64> {
        match self {
            Obj::F64(x) => Ok(x),
            _ => Err(MismatchedType)
        }
    }

    pub fn is_str(&self) -> JtsErr<&String> {
        match self {
            Obj::Str(x) => Ok(x),
            _ => Err(MismatchedType)
        }
    }

    pub fn is_node(&self) -> JtsErr<&Node> {
        match self {
            Obj::Node(x) => Ok(x),
            _ => Err(MismatchedType)
        }
    }

    pub fn new_const<T: TypeId>(val: T) -> Obj {
        val.as_variant()
    }

    pub fn new_bridge(bridge: Bridge) -> Obj {
        Obj::FnBridge(FnBridge { bridge: bridge })
    }

    pub fn set(&mut self, other: &Obj) {
        *self = other.clone();
    }

    pub fn set_to<T: TypeId>(&mut self, other: T) {
        *self = other.as_variant();
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
            Obj::FnRust() => "rust".to_string(),
            Obj::FnNative(_) => "native".to_string(),
            Obj::FnBridge(_) => "bridge".to_string(),
            Obj::Node(_) => "node".to_string(),
            Obj::Ref(_) => "quote".to_string(),
            Obj::Nil() => String::from("nil"),     
        }
    }

    pub fn cast_u32(&self) -> u32 {
        match *self {
            Obj::U32(x) => x as u32,
            Obj::U64(x) => x as u32,
            Obj::I32(x) => x as u32,
            Obj::I64(x) => x as u32,
            Obj::F32(x) => x as u32,
            Obj::F64(x) => x as u32,
            _ => u32::default(),
        }
    }

    pub fn cast_u64(&self) -> u64 {
        match *self {
            Obj::U32(x) => x as u64,
            Obj::U64(x) => x as u64,
            Obj::I32(x) => x as u64,
            Obj::I64(x) => x as u64,
            Obj::F32(x) => x as u64,
            Obj::F64(x) => x as u64,
            _ => u64::default(),
        }
    }

    pub fn cast_i32(&self) -> i32 {
        match *self {
            Obj::U32(x) => x as i32,
            Obj::U64(x) => x as i32,
            Obj::I32(x) => x as i32,
            Obj::I64(x) => x as i32,
            Obj::F32(x) => x as i32,
            Obj::F64(x) => x as i32,
            _ => i32::default(),
        }
    }

    pub fn cast_i64(&self) -> i64 {
        match *self {
            Obj::U32(x) => x as i64,
            Obj::U64(x) => x as i64,
            Obj::I32(x) => x as i64,
            Obj::I64(x) => x as i64,
            Obj::F32(x) => x as i64,
            Obj::F64(x) => x as i64,
            _ => i64::default(),
        }
    }

    pub fn cast_f32(&self) -> f32 {
        match *self {
            Obj::U32(x) => x as f32,
            Obj::U64(x) => x as f32,
            Obj::I32(x) => x as f32,
            Obj::I64(x) => x as f32,
            Obj::F32(x) => x as f32,
            Obj::F64(x) => x as f32,
            _ => f32::default(),
        }
    }

    pub fn cast_f64(&self) -> f64 {
        match *self {
            Obj::U32(x) => x as f64,
            Obj::U64(x) => x as f64,
            Obj::I32(x) => x as f64,
            Obj::I64(x) => x as f64,
            Obj::F32(x) => x as f64,
            Obj::F64(x) => x as f64,
            _ => f64::default(),
        }
    }
}