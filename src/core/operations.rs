use super::{
    objects::Obj,
    objects::Obj::*,
};

// TODO: add error checking to all operations and casts

impl Obj {
    pub fn add(&mut self, other: Obj) {
        match self {
            U32(x) => *x += other.as_u32(),
            U64(x) => *x += other.as_u64(),
            I32(x) => *x += other.as_i32(),
            I64(x) => *x += other.as_i64(),
            F32(x) => *x += other.as_f32(),
            F64(x) => *x += other.as_f64(),
            _ => (),
        };
    }

    pub fn sub(&mut self, other: Obj) {
        match self {
            U32(x) => *x -= other.as_u32(),
            U64(x) => *x -= other.as_u64(),
            I32(x) => *x -= other.as_i32(),
            I64(x) => *x -= other.as_i64(),
            F32(x) => *x -= other.as_f32(),
            F64(x) => *x -= other.as_f64(),
            _ => (),
        };
    }

    pub fn mul(&mut self, other: Obj) {
        match self {
            U32(x) => *x *= other.as_u32(),
            U64(x) => *x *= other.as_u64(),
            I32(x) => *x *= other.as_i32(),
            I64(x) => *x *= other.as_i64(),
            F32(x) => *x *= other.as_f32(),
            F64(x) => *x *= other.as_f64(),
            _ => (),
        };
    }

    pub fn div(&mut self, other: Obj) {
        match self {
            U32(x) => *x /= other.as_u32(),
            U64(x) => *x /= other.as_u64(),
            I32(x) => *x /= other.as_i32(),
            I64(x) => *x /= other.as_i64(),
            F32(x) => *x /= other.as_f32(),
            F64(x) => *x /= other.as_f64(),
            _ => (),
        };
    }

    pub fn modulos(&mut self, other: Obj) {
        match self {
            U32(x) => *x %= other.as_u32(),
            U64(x) => *x %= other.as_u64(),
            I32(x) => *x %= other.as_i32(),
            I64(x) => *x %= other.as_i64(),
            F32(x) => *x %= other.as_f32(),
            F64(x) => *x %= other.as_f64(),
            _ => (),
        };
    }
}