use super::{
    objects::Obj,
    objects::Obj::*,

    err::{
        JtsErr,
        ErrType::*,
    }
};

impl Obj {
    pub fn add(&mut self, other: Obj) -> JtsErr {
        match self {
            U32(x) => *x += other.as_u32()?,
            U64(x) => *x += other.as_u64()?,
            I32(x) => *x += other.as_i32()?,
            I64(x) => *x += other.as_i64()?,
            F32(x) => *x += other.as_f32()?,
            F64(x) => *x += other.as_f64()?,
            _ => return Err(MismatchedTypes),
        }

        Ok(())
    }

    pub fn sub(&mut self, other: Obj) -> JtsErr {
        match self {
            U32(x) => *x -= other.as_u32()?,
            U64(x) => *x -= other.as_u64()?,
            I32(x) => *x -= other.as_i32()?,
            I64(x) => *x -= other.as_i64()?,
            F32(x) => *x -= other.as_f32()?,
            F64(x) => *x -= other.as_f64()?,
            _ => return Err(MismatchedTypes),
        }

        Ok(())
    }

    pub fn mul(&mut self, other: Obj) -> JtsErr {
        match self {
            U32(x) => *x *= other.as_u32()?,
            U64(x) => *x *= other.as_u64()?,
            I32(x) => *x *= other.as_i32()?,
            I64(x) => *x *= other.as_i64()?,
            F32(x) => *x *= other.as_f32()?,
            F64(x) => *x *= other.as_f64()?,
            _ => return Err(MismatchedTypes),
        }

        Ok(())
    }

    pub fn div(&mut self, other: Obj) -> JtsErr {
        match self {
            U32(x) => *x /= other.as_u32()?,
            U64(x) => *x /= other.as_u64()?,
            I32(x) => *x /= other.as_i32()?,
            I64(x) => *x /= other.as_i64()?,
            F32(x) => *x /= other.as_f32()?,
            F64(x) => *x /= other.as_f64()?,
            _ => return Err(MismatchedTypes),
        }

        Ok(())
    }

    pub fn modulos(&mut self, other: Obj) -> JtsErr {
        match self {
            U32(x) => *x %= other.as_u32()?,
            U64(x) => *x %= other.as_u64()?,
            I32(x) => *x %= other.as_i32()?,
            I64(x) => *x %= other.as_i64()?,
            F32(x) => *x %= other.as_f32()?,
            F64(x) => *x %= other.as_f64()?,
            _ => return Err(MismatchedTypes),
        }

        Ok(())
    }

    pub fn eq(&self, other: &Obj) -> JtsErr<bool> {
        match (self, other) {
            (Bool(b1), Bool(b2)) => Ok(b1 == b2),
            (Str(s1), Str(s2)) => Ok(s1 == s2),
            
            _ => {
                match (self.is_num(), other.is_num()) {
                    (Ok(n1), Ok(n2)) => {
                        Ok(n1 == n2)
                    }
                    _ => Err(IncomparableTypes)
                }
            }
        }
    }

    pub fn le(&self, other: &Obj) -> JtsErr<bool> {
        match (self.is_num(), other.is_num()) {
            (Ok(n1), Ok(n2)) => Ok(n1 < n2),
            _ => Err(IncomparableTypes)
        }
    }

    pub fn le_eq(&self, other: &Obj) -> JtsErr<bool> {
        match (self.is_num(), other.is_num()) {
            (Ok(n1), Ok(n2)) => Ok(n1 <= n2),
            _ => Err(IncomparableTypes)
        }
    }
}