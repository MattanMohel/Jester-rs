
use super:: {
    nodes::Node,

    env::{
        Shared, 
        Env
    },
    
    types::TypeId, 

    functions::{
        FnStaticImpl,
        TupleCast, 
        FnNative, 
        FnBridge, 
        FnStatic, 
        Bridge, 
        Static, 
    }, 
};

#[derive(Clone)]
pub enum Obj {
    // primitves
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),

    Bool(bool),

    // heaps
    Str(String),

    // functions
    FnStatic(FnStatic),
    FnNative(FnNative),
    FnBridge(FnBridge),

    Node(Node),
    Lazy(Shared<Obj>),

    Nil(),
}

impl Default for Obj {
    fn default() -> Self {
        Obj::Nil()
    }
}

impl Obj {
    pub fn new_const<T: TypeId>(val: T) -> Obj {
        val.into_obj()
    }

    pub fn new_bridge(bridge: Bridge) -> Obj {
        Obj::FnBridge(FnBridge { func: bridge })
    }

    pub fn new_static<A, R>(static_fn: Static<A, R>) -> Obj 
        where A: 'static + TupleCast, R: 'static + TypeId 
    {
        let static_impl = FnStaticImpl::<A, R> { func: static_fn };
        Obj::FnStatic(FnStatic { func: Box::new(static_impl) })
    }

    pub fn set(&mut self, other: &Obj) {
        *self = other.clone();
    }

    pub fn set_to<T: TypeId>(&mut self, other: T) {
        *self = other.into_obj();
    } 
    
    pub fn to_string(&self, env: &Env) -> String {
        match self {
            Obj::F32(x) => x.to_string(),
            Obj::F64(x) => x.to_string(),
            Obj::U32(x) => x.to_string(),
            Obj::U64(x) => x.to_string(),
            Obj::I32(x) => x.to_string(),
            Obj::I64(x) => x.to_string(),
            Obj::Str(x) => x.clone(),
            Obj::Bool(x) => x.to_string(),

            Obj::Lazy(x) => match env.symbol_id(x) {
                Some(sym) => sym,
                None => String::from("unknown-symbol")
            },

            Obj::Node(node) => format!("({})", node.into_iter()
                .fold(String::new(), |acc, o| {
                    if acc.is_empty() {
                        format!("{}", o.to_string(env))
                    } else {
                        format!("{} {}", acc, o.to_string(env))
                    }})),

            Obj::FnStatic(_) => "<static>".to_string(),
            Obj::FnNative(_) => "<native>".to_string(),
            Obj::FnBridge(_) => "<bridge>".to_string(),
            
            Obj::Nil() => String::from("nil"),     
        }
    }
}