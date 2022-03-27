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

///////////////////////////
/////Parse-Time Errors/////
///////////////////////////

pub type ParseErr<T = ()> = Result<T, ParseErrType>;

#[derive(Debug)]
pub enum ParseErrType {
    // symbol does not exist
    NonSym,
    // symbol is a duplicate
    DupSym,
    // symbol is disallowed
    DisSym,

    // module does not exist
    NonMod,
    // module is a duplicate
    DupMod,
    
    // parentheses are unbalanced
    Unbalanced,
    
    // generic IO errors
    IoErr,
}

impl Error for ParseErrType {}

impl Display for ParseErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse Error!")
    }
}

impl From<io::Error> for ParseErrType {
    fn from(cause: io::Error) -> Self {
        Self::IoErr
    }
}

/////////////////////////////
/////Compile-Time Errors/////
/////////////////////////////

enum EvalErrType {
    MismatchedType,
    ConstSym,
}