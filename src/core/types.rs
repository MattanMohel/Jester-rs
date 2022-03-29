use super::{objects::Obj, functions::FnNative};

pub trait TypeId {
    fn as_variant(self) -> Obj;
}

impl TypeId for u32 {
    fn as_variant(self) -> Obj {
        Obj::U32(self)
    }
}
impl TypeId for u64 {
    fn as_variant(self) -> Obj {
        Obj::U64(self)
    }
}
impl TypeId for i32 {
    fn as_variant(self) -> Obj {
        Obj::I32(self)
    }
}
impl TypeId for i64 {
    fn as_variant(self) -> Obj {
        Obj::I64(self)
    }
}
impl TypeId for f32 {
    fn as_variant(self) -> Obj {
        Obj::F32(self)
    }
}
impl TypeId for f64 {
    fn as_variant(self) -> Obj {
        Obj::F64(self)
    }
}

impl TypeId for String {
    fn as_variant(self) -> Obj {
        Obj::Str(self.clone())
    }
}

impl TypeId for FnNative {
    fn as_variant(self) -> Obj {
        Obj::FnNative(self)
    }
}

impl TypeId for () {
    fn as_variant(self) -> Obj {
        Obj::Nil()
    }
}
