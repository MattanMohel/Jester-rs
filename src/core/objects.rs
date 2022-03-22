
use super:: {
    nodes::Node,
    types::TypeId,
    functions::{FnBridge, FnNative}, 
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

    Args(Node),
    Ref(usize),

    Nil(),
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

use super::objects::Obj::*;

impl Obj {

    pub fn new_const<'a, T: TypeId>(val: T) -> Obj {
        val.as_variant()
    }

    pub fn set(&mut self, other: &Obj) {
        *self = other.clone();
    }

    pub fn set_to<T: TypeId>(&mut self, other: T) {
        *self = other.as_variant();
    } 
    
    pub fn to_string(&self) -> String {
        match self {
            F32(x) => x.to_string(),
            F64(x) => x.to_string(),
            U32(x) => x.to_string(),
            U64(x) => x.to_string(),
            I32(x) => x.to_string(),
            I64(x) => x.to_string(),
            Nil() => String::from("nil"),     
            Str(x) => x.clone(),

            _ => String::new()
        }
    }

    pub fn add(&mut self, other: &Obj) {
        match self {
            U32(x) => *x += other.cast_u32(),
            U64(x) => *x += other.cast_u64(),
            I32(x) => *x += other.cast_i32(),
            I64(x) => *x += other.cast_i64(),
            F32(x) => *x += other.cast_f32(),
            F64(x) => *x += other.cast_f64(),
            _ => (),
        };
    }

    pub fn sub(&mut self, other: &Obj) {
        match self {
            U32(x) => *x -= other.cast_u32(),
            U64(x) => *x -= other.cast_u64(),
            I32(x) => *x -= other.cast_i32(),
            I64(x) => *x -= other.cast_i64(),
            F32(x) => *x -= other.cast_f32(),
            F64(x) => *x -= other.cast_f64(),
            _ => (),
        };
    }

    pub fn mul(&mut self, other: &Obj) {
        match self {
            U32(x) => *x *= other.cast_u32(),
            U64(x) => *x *= other.cast_u64(),
            I32(x) => *x *= other.cast_i32(),
            I64(x) => *x *= other.cast_i64(),
            F32(x) => *x *= other.cast_f32(),
            F64(x) => *x *= other.cast_f64(),
            _ => (),
        };
    }

    pub fn div(&mut self, other: &Obj) {
        match self {
            U32(x) => *x /= other.cast_u32(),
            U64(x) => *x /= other.cast_u64(),
            I32(x) => *x /= other.cast_i32(),
            I64(x) => *x /= other.cast_i64(),
            F32(x) => *x /= other.cast_f32(),
            F64(x) => *x /= other.cast_f64(),
            _ => (),
        };
    }

    pub fn cast_u32(&self) -> u32 {
        match *self {
            U32(x) => x as u32,
            U64(x) => x as u32,
            I32(x) => x as u32,
            I64(x) => x as u32,
            F32(x) => x as u32,
            F64(x) => x as u32,
            _ => u32::default(),
        }
    }

    pub fn cast_u64(&self) -> u64 {
        match *self {
            U32(x) => x as u64,
            U64(x) => x as u64,
            I32(x) => x as u64,
            I64(x) => x as u64,
            F32(x) => x as u64,
            F64(x) => x as u64,
            _ => u64::default(),
        }
    }

    pub fn cast_i32(&self) -> i32 {
        match *self {
            U32(x) => x as i32,
            U64(x) => x as i32,
            I32(x) => x as i32,
            I64(x) => x as i32,
            F32(x) => x as i32,
            F64(x) => x as i32,
            _ => i32::default(),
        }
    }

    pub fn cast_i64(&self) -> i64 {
        match *self {
            U32(x) => x as i64,
            U64(x) => x as i64,
            I32(x) => x as i64,
            I64(x) => x as i64,
            F32(x) => x as i64,
            F64(x) => x as i64,
            _ => i64::default(),
        }
    }

    pub fn cast_f32(&self) -> f32 {
        match *self {
            U32(x) => x as f32,
            U64(x) => x as f32,
            I32(x) => x as f32,
            I64(x) => x as f32,
            F32(x) => x as f32,
            F64(x) => x as f32,
            _ => f32::default(),
        }
    }

    pub fn cast_f64(&self) -> f64 {
        match *self {
            U32(x) => x as f64,
            U64(x) => x as f64,
            I32(x) => x as f64,
            I64(x) => x as f64,
            F32(x) => x as f64,
            F64(x) => x as f64,
            _ => f64::default(),
        }
    }
}

pub struct ObjData {
    pub is_pub:    bool,
    pub is_const:  bool,
    pub ref_count: usize,
}