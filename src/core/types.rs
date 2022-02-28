use super::objects::Obj;

#[derive(Copy, Clone)]
pub enum Type {
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Nil(),
}

pub trait TypeId: Copy {
    fn as_variant(&self) -> Type;
}

impl TypeId for u32 {
    fn as_variant(&self) -> Type {
        Type::U32(*self)
    }
}
impl TypeId for u64 {
    fn as_variant(&self) -> Type {
        Type::U64(*self)
    }
}
impl TypeId for i32 {
    fn as_variant(&self) -> Type {
        Type::I32(*self)
    }
}
impl TypeId for i64 {
    fn as_variant(&self) -> Type {
        Type::I64(*self)
    }
}
impl TypeId for f32 {
    fn as_variant(&self) -> Type {
        Type::F32(*self)
    }
}
impl TypeId for f64 {
    fn as_variant(&self) -> Type {
        Type::F64(*self)
    }
}

impl TypeId for () {
    fn as_variant(&self) -> Type {
        Type::Nil()
    }
}
