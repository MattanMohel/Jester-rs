use super::{
    objects::Obj,
    objects::Obj::*,
};
impl Obj {
    pub fn add(&mut self, other: Obj) {
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

    pub fn sub(&mut self, other: Obj) {
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

    pub fn mul(&mut self, other: Obj) {
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

    pub fn div(&mut self, other: Obj) {
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
}