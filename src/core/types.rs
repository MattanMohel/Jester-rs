use super::objects::{Obj, Node};
use super::env::Env;

pub type NativeFn = fn(&mut Env, *mut Node) -> Obj;

#[derive(Clone)]
pub enum Type {
    // primitve types
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),

    // callable types
    Native(NativeFn),

    // heap types
    Str(String),
    Ref(*mut Obj),
    Node(*mut Node),

    Nil(),
}

pub trait TypeId {
    fn as_variant(&self) -> Type;
}

impl TypeId for *mut Obj {
    fn as_variant(&self) -> Type {
        Type::Ref(*self)
    }
}

impl TypeId for *mut Node {
    fn as_variant(&self) -> Type {
        Type::Node(*self)
    }
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

impl TypeId for String {
    fn as_variant(&self) -> Type {
        Type::Str(self.clone())
    }
}

impl TypeId for () {
    fn as_variant(&self) -> Type {
        Type::Nil()
    }
}
