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
    /// asserts that a symbol cannot
    /// be found in the given context
    MissingSymbol,
    /// asserts that a symbol is a 
    /// duplicate, meaning its already
    /// defined in the given context
    DuplicateSymbol,
    /// asserts that a symbol is disallowed,
    /// meaning that it conflicts with possible
    /// internal symbols such as 'gensym'
    DisallowedSymbol,
    /// asserts that a module cannot
    /// be found in the given context
    MissingModule,
    /// asserts that a module is already
    /// defined in the environment
    DuplicateModule, 
    /// asserts that an input's parentheses 
    /// were unbalanced, meaning the ratio of 
    /// right to left parentheses was non-equal
    UnbalancedParentheses,  
    /// an Io error represented in Jts for
    /// error modularity
    IoErr,
    /// asserts that the prescribed entry 
    /// module could not be found. Most usually
    /// the entry module is 'main'
    NoEntry,
    /// asserts that a given pair of types
    /// do not match and could not be operated
    /// upon correctly in the given context
    MismatchedTypes,
    /// asserts that a given pair of types
    /// could not be compared
    IncomparableTypes,
    /// asserts that a given type could not
    /// be cast into the other successfully 
    ErrCastTypes,
    /// asserts that the program attempted to
    /// mutate a constant value
    ConstAssign,
    /// asserts that the program attempted to 
    /// invoke a non-callable value
    NonCallable,
    /// asserts that the program attempted to 
    /// query a value from outside the bounds
    /// of a given collection
    OutOfBounds,
    /// asserts that a given input parameter 
    /// list to a function invocation could 
    /// not match with the function signature
    UnmatchedParamLists,
}

impl Error for JtsErrType {}

impl Display for JtsErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Toolchain Error!")
    }
}

impl From<io::Error> for JtsErrType {
    fn from(_: io::Error) -> Self {
        Self::IoErr
    }
}