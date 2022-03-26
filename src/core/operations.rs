
use std::{cell::RefCell, ops::DerefMut};

use super::{
    objects::Obj,
    objects::Obj::*,
};

pub fn add_to(obj: &mut Obj, other: &Obj) {
    match obj {
        U32(x) => *x += other.cast_u32(),
        U64(x) => *x += other.cast_u64(),
        I32(x) => *x += other.cast_i32(),
        I64(x) => *x += other.cast_i64(),
        F32(x) => *x += other.cast_f32(),
        F64(x) => *x += other.cast_f64(),
        _ => (),
    };
}

pub fn sub_to(obj: &mut Obj, other: &Obj) {
    match obj {
        U32(x) => *x -= other.cast_u32(),
        U64(x) => *x -= other.cast_u64(),
        I32(x) => *x -= other.cast_i32(),
        I64(x) => *x -= other.cast_i64(),
        F32(x) => *x -= other.cast_f32(),
        F64(x) => *x -= other.cast_f64(),
        _ => (),
    };
}

pub fn mul_to(obj: &mut Obj, other: &Obj) {
    match obj {
        U32(x) => *x *= other.cast_u32(),
        U64(x) => *x *= other.cast_u64(),
        I32(x) => *x *= other.cast_i32(),
        I64(x) => *x *= other.cast_i64(),
        F32(x) => *x *= other.cast_f32(),
        F64(x) => *x *= other.cast_f64(),
        _ => (),
    };
}

pub fn div_to(obj: &mut Obj, other: &Obj) {
    match obj {
        U32(x) => *x /= other.cast_u32(),
        U64(x) => *x /= other.cast_u64(),
        I32(x) => *x /= other.cast_i32(),
        I64(x) => *x /= other.cast_i64(),
        F32(x) => *x /= other.cast_f32(),
        F64(x) => *x /= other.cast_f64(),
        _ => (),
    };
}