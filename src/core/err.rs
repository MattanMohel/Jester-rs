use std::{
    error::Error,
    fmt::Display,
    io,
};

pub trait AsResult<U = Self> {
    // converts state of self into result
    fn as_result<O, E>(&self, ok: O, err: E) -> Result<O, E> 
        where E: Error;

    // converts state of !self into result
    fn as_result_rev<O, E>(&self, ok: O, err: E) -> Result<O, E> 
        where E: Error;
    
    // converts self into self-result value 'U'
    fn into_result<E>(self, err: E) -> Result<U, E> 
        where E: Error;

    // asserts state of self and converts into result
    fn assert<E>(&self, err: E) -> Result<(), E>
        where E: Error;
}

impl AsResult for bool {
    fn as_result<O, E>(&self, ok: O, err: E) -> Result<O, E> {
        if *self {
            Ok(ok)
        } else {
            Err(err)
        }
    }

    fn as_result_rev<O, E>(&self, ok: O, err: E) -> Result<O, E> {
        if !self {
            Ok(ok)
        } else {
            Err(err)
        }
    }

    fn into_result<E>(self, err: E) -> Result<Self, E> {
        if self {
            Ok(self)
        } else {
            Err(err)
        }
    }

    fn assert<E>(&self, err: E) -> Result<(), E> {
        if *self {
            Ok(())
        } else {
            Err(err)
        }
    }
}

impl<T> AsResult<T> for Option<T> {
    fn as_result<O, E>(&self, ok: O, err: E) -> Result<O, E> {
        match self {
            Some(_) => Ok(ok),
            None => Err(err),
        }
    }

    fn as_result_rev<O, E>(&self, ok: O, err: E) -> Result<O, E> {
        match self {
            Some(_) => Err(err),
            None => Ok(ok),
        }
    }

    fn into_result<E>(self, err: E) -> Result<T, E> {
        match self {
            Some(ok) => Ok(ok),
            None => Err(err)
        }
    }

    fn assert<E>(&self, err: E) -> Result<(), E> {
        match self {
            Some(_) => Ok(()),
            None => Err(err),
        }
    }
}

/////////////////////////////////////
/////Jester-Script Error Defines/////
/////////////////////////////////////

pub type JtsErr<T = ()> = Result<T, JtsErrType>;

#[derive(Debug)]
pub enum JtsErrType {
    /// standard Rust IO error
    IoErr(String),
    /// symbol not found in current context
    MissingSymbol,
    /// specified symbol already defined
    DuplicateSymbol,
    /// symbol structure is not allowed
    DisallowedSymbol,
    /// module not found in current context
    MissingModule,
    /// specified module already defined
    DuplicateModule, 
    /// counts of '(' and ')' are not equal
    UnbalancedParentheses,  
    /// given types cannot be inter-operated
    MismatchedType,
    /// given types cannot be compared
    UncomparableType,
    /// given type cannot be cast into another
    ErrCastType,
    /// assigned to const value
    ConstAssign,
    /// queried an index out of bounds from its collection
    OutOfBounds,
    /// input params do not match expected params
    UnmatchedParamLists,
    /// placement of special op erroneous
    ErrSpecialOp,
    /// attempted to access and uninitialized env
    UninitEnv,
    /// a runtime assert
    RuntimeAssert
}

impl Error for JtsErrType {}

impl Display for JtsErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Toolchain Error!")
    }
}

impl From<io::Error> for JtsErrType {
    fn from(err: io::Error) -> Self {
        Self::IoErr(err.to_string())
    }
}