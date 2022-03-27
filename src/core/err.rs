use super::{objects::Obj, env::Shared};

use std::error::Error;
use std::fmt::Display;
use std::io;

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
        if !(*self) {
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

#[derive(Debug)]
pub enum ParseErr {
    NonSym(String),
    DupSym(String),
    NonMod(String),
    DupMod(String),

    IoErr(String),

    Unbalanced(isize),
}

impl Error for ParseErr {}

impl Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErr::NonSym(s) => write!(f, "tried accessing undeclared symbol '{}'!", s),
            ParseErr::DupSym(s) => write!(f, "module already contains symbol '{}'!", s),
            ParseErr::NonMod(s) => write!(f, "tried accessing undeclared module '{}'!", s),
            ParseErr::DupMod(s) => write!(f, "env already contains module '{}'!", s),

            ParseErr::IoErr(s) => write!(f, "filepath '{}' doesn't exist", s),

            ParseErr::Unbalanced(n) => if n.is_positive() {
                write!(f, "too many '(' => code is unbalanced")
            } else {
                write!(f, "too many ')' => code is unbalanced")
            }
        }
    }
}

impl From<io::Error> for ParseErr {
    fn from(cause: io::Error) -> Self {
        Self::IoErr(cause.to_string())
    }
}

/////////////////////////////
/////Compile-Time Errors/////
/////////////////////////////

struct EvalErr {
    op:  Shared<Obj>,
    opr: Shared<Obj>,

    line: usize,

    err_type: EvalErrType,
}

enum EvalErrType {
    MismatchedType,
    ConstSym,
}