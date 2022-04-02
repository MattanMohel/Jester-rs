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

pub type JtsErr<T = ()> = Result<T, ErrType>;

#[derive(Debug)]
pub enum ErrType {
    Todo,
    // symbol does not exist
    MissingSymbol,
    // symbol is a duplicate
    DuplicateSymbol,
    // symbol is disallowed
    DisallowedSymbol,
    // module does not exist
    MissingModule,
    // module is a duplicate
    DuplicateModule, 
    // parentheses are unbalanced
    UnbalancedParentheses,  
    // generic IO errors
    IoErr,
    // Missing 'main' function
    NoEntry,
    // types cannot match
    MismatchedType,
    // assigning to const value
    ConstAssign,
    // tried executing a non-callable
    NonCallable,
    // index out of bounds
    OutOfBounds,
}

impl Error for ErrType {}

impl Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Toolchain Error!")
    }
}

impl From<io::Error> for ErrType {
    fn from(cause: io::Error) -> Self {
        Self::IoErr
    }
}