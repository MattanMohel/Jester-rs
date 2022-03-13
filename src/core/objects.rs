
use super::types::{Type, TypeId};

pub type Node = Vec<usize>;

#[derive(Clone)]
pub struct Obj {
    pub typ: Type,
}

impl Obj {
    pub fn new(typ: Type) -> Obj {
        Obj { typ }
    }

    pub fn set(&mut self, other: &Obj) {
        *self = other.clone();
    }

    pub fn set_to<T: TypeId>(&mut self, other: T) {
        self.typ = other.as_variant();
    } 
    
    pub fn new_const<T: TypeId>(val: T) -> Obj {
        Obj::new(val.as_variant())
    }

    pub fn add(&mut self, other: &Obj) {
        match &mut self.typ {
            Type::U32(x) => *x += other.cast_u32(),
            Type::U64(x) => *x += other.cast_u64(),
            Type::I32(x) => *x += other.cast_i32(),
            Type::I64(x) => *x += other.cast_i64(),
            Type::F32(x) => *x += other.cast_f32(),
            Type::F64(x) => *x += other.cast_f64(),
            _ => (),
        };
    }

    pub fn sub(&mut self, other: &Obj) {
        match &mut self.typ {
            Type::U32(x) => *x -= other.cast_u32(),
            Type::U64(x) => *x -= other.cast_u64(),
            Type::I32(x) => *x -= other.cast_i32(),
            Type::I64(x) => *x -= other.cast_i64(),
            Type::F32(x) => *x -= other.cast_f32(),
            Type::F64(x) => *x -= other.cast_f64(),
            _ => (),
        };
    }

    pub fn mul(&mut self, other: &Obj) {
        match &mut self.typ {
            Type::U32(x) => *x *= other.cast_u32(),
            Type::U64(x) => *x *= other.cast_u64(),
            Type::I32(x) => *x *= other.cast_i32(),
            Type::I64(x) => *x *= other.cast_i64(),
            Type::F32(x) => *x *= other.cast_f32(),
            Type::F64(x) => *x *= other.cast_f64(),
            _ => (),
        };
    }

    pub fn div(&mut self, other: &Obj) {
        match &mut self.typ {
            Type::U32(x) => *x /= other.cast_u32(),
            Type::U64(x) => *x /= other.cast_u64(),
            Type::I32(x) => *x /= other.cast_i32(),
            Type::I64(x) => *x /= other.cast_i64(),
            Type::F32(x) => *x /= other.cast_f32(),
            Type::F64(x) => *x /= other.cast_f64(),
            _ => (),
        };
    }

    pub fn cast_u32(&self) -> u32 {
        match self.typ {
            Type::U32(x) => x as u32,
            Type::U64(x) => x as u32,
            Type::I32(x) => x as u32,
            Type::I64(x) => x as u32,
            Type::F32(x) => x as u32,
            Type::F64(x) => x as u32,
            _ => u32::default(),
        }
    }

    pub fn cast_u64(&self) -> u64 {
        match self.typ {
            Type::U32(x) => x as u64,
            Type::U64(x) => x as u64,
            Type::I32(x) => x as u64,
            Type::I64(x) => x as u64,
            Type::F32(x) => x as u64,
            Type::F64(x) => x as u64,
            _ => u64::default(),
        }
    }

    pub fn cast_i32(&self) -> i32 {
        match self.typ {
            Type::U32(x) => x as i32,
            Type::U64(x) => x as i32,
            Type::I32(x) => x as i32,
            Type::I64(x) => x as i32,
            Type::F32(x) => x as i32,
            Type::F64(x) => x as i32,
            _ => i32::default(),
        }
    }

    pub fn cast_i64(&self) -> i64 {
        match self.typ {
            Type::U32(x) => x as i64,
            Type::U64(x) => x as i64,
            Type::I32(x) => x as i64,
            Type::I64(x) => x as i64,
            Type::F32(x) => x as i64,
            Type::F64(x) => x as i64,
            _ => i64::default(),
        }
    }

    pub fn cast_f32(&self) -> f32 {
        match self.typ {
            Type::U32(x) => x as f32,
            Type::U64(x) => x as f32,
            Type::I32(x) => x as f32,
            Type::I64(x) => x as f32,
            Type::F32(x) => x as f32,
            Type::F64(x) => x as f32,
            _ => f32::default(),
        }
    }

    pub fn cast_f64(&self) -> f64 {
        match self.typ {
            Type::U32(x) => x as f64,
            Type::U64(x) => x as f64,
            Type::I32(x) => x as f64,
            Type::I64(x) => x as f64,
            Type::F32(x) => x as f64,
            Type::F64(x) => x as f64,
            _ => f64::default(),
        }
    }
}