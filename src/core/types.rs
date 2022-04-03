use super::{
    nodes::Node,
    env::Shared,
    objects::Obj, 
    functions::FnNative, 
    
    err::{
        JtsErr,
        JtsErrType::*,
    }, 
};

pub trait TypeId: Default {
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

impl TypeId for bool {
    fn as_variant(self) -> Obj {
        Obj::Bool(self)
    }
}

impl TypeId for String {
    fn as_variant(self) -> Obj {
        Obj::Str(self)
    }
}

impl TypeId for FnNative {
    fn as_variant(self) -> Obj {
        Obj::FnNative(self)
    }
}

impl TypeId for Node {
    fn as_variant(self) -> Obj {
        Obj::Node(self)
    }
}

impl TypeId for Shared<Obj> {
    fn as_variant(self) -> Obj {
        Obj::Lazy(self)
    }
}

impl TypeId for () {
    fn as_variant(self) -> Obj {
        Obj::Nil()
    }
}

impl Obj {
    pub fn is_num(&self) -> JtsErr<f64> {
        match self {
            Obj::U32(x) => Ok(*x as f64),
            Obj::U64(x) => Ok(*x as f64),
            Obj::I32(x) => Ok(*x as f64),
            Obj::I64(x) => Ok(*x as f64),
            Obj::F32(x) => Ok(*x as f64),
            Obj::F64(x) => Ok(*x as f64),
            _ => Err(MismatchedTypes)
        }
    } 

    pub fn is_int(&self) -> JtsErr<u64> {
        match self {
            Obj::U32(x) => Ok(*x as u64),
            Obj::U64(x) => Ok(*x as u64),
            Obj::I32(x) => Ok(*x as u64),
            Obj::I64(x) => Ok(*x as u64),
            _ => Err(MismatchedTypes)
        }
    } 

    pub fn is_u32(&self) -> JtsErr<&u32> {
        match self {
            Obj::U32(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }
 
    pub fn is_u32_mut(&mut self) -> JtsErr<&mut u32> {
        match self {
            Obj::U32(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_u64(&self) -> JtsErr<&u64> {
        match self {
            Obj::U64(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_u64_mut(&mut self) -> JtsErr<&mut u64> {
        match self {
            Obj::U64(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_i32(&self) -> JtsErr<&i32> {
        match self {
            Obj::I32(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_i32_mut(&mut self) -> JtsErr<&mut i32> {
        match self {
            Obj::I32(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_i64(&self) -> JtsErr<&i64> {
        match self {
            Obj::I64(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_i64_mut(&mut self) -> JtsErr<&mut i64> {
        match self {
            Obj::I64(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_f32(&self) -> JtsErr<&f32> {
        match self {
            Obj::F32(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_f32_mut(&mut self) -> JtsErr<&mut f32> {
        match self {
            Obj::F32(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_f64(&self) -> JtsErr<&f64> {
        match self {
            Obj::F64(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_f64_mut(&mut self) -> JtsErr<&mut f64> {
        match self {
            Obj::F64(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_bool(&self) -> JtsErr<&bool> {
        match self {
            Obj::Bool(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_bool_mut(&mut self) -> JtsErr<&mut bool> {
        match self {
            Obj::Bool(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_str(&self) -> JtsErr<&String> {
        match self {
            Obj::Str(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_str_mut(&mut self) -> JtsErr<&mut String> {
        match self {
            Obj::Str(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_node(&self) -> JtsErr<&Node> {
        match self {
            Obj::Node(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_node_mut(&mut self) -> JtsErr<&mut Node> {
        match self {
            Obj::Node(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_lazy(&self) -> JtsErr<&Shared<Obj>> {
        match self {
            Obj::Lazy(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn is_lazy_mut(&mut self) -> JtsErr<&mut Shared<Obj>> {
        match self {
            Obj::Lazy(x) => Ok(x),
            _ => Err(MismatchedTypes)
        }
    }

    pub fn as_u32(&self) -> JtsErr<u32> {
        match *self {
            Obj::U32(x) => Ok(x as u32),
            Obj::U64(x) => Ok(x as u32),
            Obj::I32(x) => Ok(x as u32),
            Obj::I64(x) => Ok(x as u32),
            Obj::F32(x) => Ok(x as u32),
            Obj::F64(x) => Ok(x as u32),
            _ => Err(ErrCastTypes),
        }
    }

    pub fn as_u64(&self) -> JtsErr<u64> {
        match *self {
            Obj::U32(x) => Ok(x as u64),
            Obj::U64(x) => Ok(x as u64),
            Obj::I32(x) => Ok(x as u64),
            Obj::I64(x) => Ok(x as u64),
            Obj::F32(x) => Ok(x as u64),
            Obj::F64(x) => Ok(x as u64),
            _ => Err(ErrCastTypes),
        }
    }

    pub fn as_i32(&self) -> JtsErr<i32> {
        match *self {
            Obj::U32(x) => Ok(x as i32),
            Obj::U64(x) => Ok(x as i32),
            Obj::I32(x) => Ok(x as i32),
            Obj::I64(x) => Ok(x as i32),
            Obj::F32(x) => Ok(x as i32),
            Obj::F64(x) => Ok(x as i32),
            _ => Err(ErrCastTypes),
        }
    }

    pub fn as_i64(&self) -> JtsErr<i64> {
        match *self {
            Obj::U32(x) => Ok(x as i64),
            Obj::U64(x) => Ok(x as i64),
            Obj::I32(x) => Ok(x as i64),
            Obj::I64(x) => Ok(x as i64),
            Obj::F32(x) => Ok(x as i64),
            Obj::F64(x) => Ok(x as i64),
            _ => Err(ErrCastTypes),
        }
    }

    pub fn as_f32(&self) -> JtsErr<f32> {
        match *self {
            Obj::U32(x) => Ok(x as f32),
            Obj::U64(x) => Ok(x as f32),
            Obj::I32(x) => Ok(x as f32),
            Obj::I64(x) => Ok(x as f32),
            Obj::F32(x) => Ok(x as f32),
            Obj::F64(x) => Ok(x as f32),
            _ => Err(ErrCastTypes),
        }
    }

    pub fn as_f64(&self) -> JtsErr<f64> {
        match *self {
            Obj::U32(x) => Ok(x as f64),
            Obj::U64(x) => Ok(x as f64),
            Obj::I32(x) => Ok(x as f64),
            Obj::I64(x) => Ok(x as f64),
            Obj::F32(x) => Ok(x as f64),
            Obj::F64(x) => Ok(x as f64),
            _ => Err(ErrCastTypes),
        }
    }
}