use std::ops::Deref;

use super::{
    env::Env, 
    err::JtsErr,
    objects::Obj,

    types::TypeId,

    nodes::{
        Node, 
        NodeIter
    },  
};

/// marks a callable Jester-Script object
/// 
/// a callable object recieves the environment 
/// plus arguments and returns a new ```Result<Obj>``` 
/// transformed by ```self```'s behaviour
/// 
/// The three types of internal callables are:
///  - Native Functions
///  - Bridge Functions
///  - Static Functions
pub trait Callable: CloneCallable {
    fn invoke(&self, env: &Env, args: &mut NodeIter) -> JtsErr<Obj>;
}

/// makes ```Box<dyn Clone>``` clonable
pub trait CloneCallable {
    fn clone_foo<'a>(&self) -> Box<dyn Callable>;
}

////////////////////////////////
/////Implementation Details/////
////////////////////////////////

impl<T> CloneCallable for T
    where T: Callable + Clone + 'static,
{
    fn clone_foo(&self) -> Box<dyn Callable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Callable> {
    fn clone(&self) -> Self {
        self.clone_foo()
    }
}

/// native functions are defined natively using 
/// Jester-Script
/// 
/// ## Example
/// 
/// ```
/// (defun add (a b)
///     (+ a b))
/// ```
/// this example creates a new function called "add"
/// which takes two arguments mapped to '(a b)'
/// 
/// Inside the function's lexical scope 'a' and 'b'
/// are summed and returned in a progn manner
#[derive(Clone, Default)]
pub struct FnNative {
    pub body: Node,
    pub params: Node,
}

impl FnNative {
    #[inline]
    pub fn invoke(&self, env: &Env, args: &mut NodeIter) -> JtsErr<Obj> {
        self.params.into_iter().scope(env, args, || {
            self.body.into_iter()
                .progn(|obj| { env.eval(obj.deref()) })
        })
    }
}

/////////////////////////
/////Bridge Function/////
/////////////////////////

pub type Bridge = fn(&Env, &mut NodeIter) -> JtsErr<Obj>;

/// bridge functions 'bridge' Jester-Script and Rust
/// through the use of objects in Rust
/// 
/// ## Example
/// 
/// ```
/// env.add_symbol("my-favorite-number", Obj::new_bridge(|env, args| {
///     Ok(Obj::new_const(21))  
/// }))
/// ```
/// this example defines a new bridge called "my-favorite-number"
/// which recieves an arbitrary argument list and returns the 
/// number 21 wrapped inside an object
/// 
/// (see more examples in src//prelude)
#[derive(Clone)]
pub struct FnBridge {
    pub func: Bridge,
}

impl Callable for FnBridge {
    fn invoke(&self, env: &Env, args: &mut NodeIter) -> JtsErr<Obj> {
        (self.func)(env, args)
    }
}

/////////////////////////
/////Static Function/////
/////////////////////////

pub type Static<A, R> = fn(A) -> R;

/// Static Functions
/// 
/// a literal function: defined in Rust 
/// and callable in Jester-Script
/// 
/// ## Example - TODO
/// 
/// ```
/// #[derive(Static)]
/// fn add((a b): (i32, i32)) -> i32 {
///     a + b
/// }
/// 
/// env.add_symbol("add", Obj::new_static(add))?;
/// ```
#[derive(Clone)]
pub struct FnStatic {
    pub func: Box<dyn Callable>
}

impl Callable for FnStatic {
    fn invoke(&self, env: &Env, args: &mut NodeIter) -> JtsErr<Obj> {
        self.func.invoke(env, args)
    }
}

#[derive(Clone)]
pub struct FnStaticImpl<A, R>
    where A: TupleCast, R: TypeId
{
    pub func: fn(A) -> R
}

impl<A, R> Callable for FnStaticImpl<A, R> 
    where A: 'static + TupleCast, R: 'static + TypeId
{
    fn invoke(&self, _: &Env, args: &mut NodeIter) -> JtsErr<Obj> {
        let map = unsafe { A::cast(args)? };
        let res = (self.func)(map);
        Ok(Obj::new_const(res))
    }
}

/// a trait allowing for the conversion
/// from ```Vec<Obj>``` to ```(T..)```
/// 
/// allows for the creation of Static Functions
/// by converting nodes of objects into tuple arguments
pub trait TupleCast 
    where Self: Sized + Clone
{
    unsafe fn cast(args: &NodeIter) -> JtsErr<Self>;
}

impl TupleCast for (i32, i32) {
    unsafe fn cast(args: &NodeIter) -> JtsErr<Self> {
        Ok((
            args.get(0)?.cast_as::<i32>()?,
            args.get(1)?.cast_as::<i32>()?
        ))
    }
}
