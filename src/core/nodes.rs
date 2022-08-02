use std::{
    ops::Deref,

    cell::{
        Ref,
        RefMut,
    }, 
};

use super::{
    objects::Obj,

    env::{
        Env,
        Shared,
        new_shared, 
    }, 
    
    err::{
        JtsErr, 
        JtsErrType::*, 
        AsResult
    }, types::TypeId,
};

#[derive(Clone)]
pub struct Node {
    pub args: Vec<Shared<Obj>>,
}

#[derive(Clone, Copy)]
pub struct NodeIter<'a> {
    pub args: &'a Vec<Shared<Obj>>,
    offset: usize,
    i: usize,
}

impl Default for Node {
    fn default() -> Self {
        Node { args: Vec::new() }
    }
}

impl<'a> IntoIterator for &'a Node {
    type Item = Ref<'a, Obj>;
    type IntoIter = NodeIter<'a>;
    
    fn into_iter(self) -> Self::IntoIter {
        NodeIter {
            args: &self.args,
            offset: 0,
            i: 0,
        }
    }
}

impl Node {
    pub fn len(&self) -> usize {
        self.args.len()
    }

    pub fn remove(&mut self, i: usize) -> JtsErr<Obj> {
        if i < self.len() {
            let elem = self.args.remove(i);
            let elem = elem.borrow();
            Ok(elem.clone())
        } else {
            Err(OutOfBounds)
        }
    }

    pub fn get(&self, i: usize) -> JtsErr<Ref<'_, Obj>> {
        match self.args.get(i) {
            Some(obj) => Ok(obj.borrow()),
            None => Err(OutOfBounds)
        }
    }

    pub fn get_mut(&self, i: usize) -> JtsErr<RefMut<'_, Obj>> {
        match self.args.get(i) {
            Some(obj) => Ok(obj.borrow_mut()),
            None => Err(OutOfBounds)
        }    }

    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    pub fn into_iter_from(&self, from: usize) -> NodeIter<'_> {
        NodeIter {
            args: &self.args,
            offset: from,
            i: 0,
        }
    }
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = Ref<'a, Obj>;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        self.args.get(self.offset + self.i - 1)
            .map(|symbol| { symbol.borrow() })
    }
}

impl<'a> NodeIter<'a> {
    pub fn len(&self) -> usize {
        self.args.len()
    }

    /// shifts the iterator offset by 1 and returns
    /// a non-owning reference to the previous index
    /// to bypass borrowing rules infringement 
    pub fn shift(&mut self) -> JtsErr<Shared<Obj>> {
        let res = 
            match self.args.get(self.offset) {
                Some(obj) => Ok(obj.clone()),
                None => Err(OutOfBounds)
            };

        self.offset += 1;
        
        res
    }

    /// returns a shared reference to the prescribed
    /// index, allowing for both mutable and immutable borrow
    pub fn get_shared(&self, i: usize) -> JtsErr<Shared<Obj>> {
        match self.args.get(self.offset + i) {
            Some(obj) => Ok(obj.clone()),
            None => Err(OutOfBounds)
        }
    } 

    // returns an immutable reference to the prescribed index
    pub fn get(&self, i: usize) -> JtsErr<Ref<'_, Obj>> {
        match self.args.get(self.offset + i) {
            Some(obj) => Ok(obj.borrow()),
            None => Err(OutOfBounds)
        }
    }

    // returns an mutable reference to the prescribed index
    pub fn get_mut(&self, i: usize) -> JtsErr<RefMut<'_, Obj>> {
        match self.args.get(self.offset + i) {
            Some(obj) => Ok(obj.borrow_mut()),
            None => Err(OutOfBounds)
        }    
    }

    /// creates a new owned Node struct with elements
    /// ranging from the iterator's 
    /// ```
    /// offset + from
    /// ``` 
    /// to the end of its range
    pub fn into_node_from(&self, from: usize) -> Node {
        Node { 
            args: self.args[self.offset + from..].iter()
                .cloned()
                .collect()
        }
    }

    //////////////////////////
    /////Iterator Methods/////
    //////////////////////////

    /// creates a lexical scope for self's elements set to the 
    /// passed arguments. The closure is executed in respect to
    /// the scope, the parameters are resets, and the evaluation 
    /// of the execution is returned
    /// 
    /// ## Example
    /// 
    /// ```
    /// (defun add (a b)
    ///     (+ a b))
    /// 
    /// 
    /// (add 1 2) 
    /// ```
    ///      
    /// - create a scope where a = nil and b = nil
    /// - set a = 1 and b = 2 (provided by arguments)
    /// - evaluate (+ a b) => (+ 1 2)
    /// - return 3 and reset a = nil and b = nil

    pub fn scope<F>(&self, env: &Env, args: &mut NodeIter, mut f: F) -> JtsErr<Obj>
        where F: FnMut() -> JtsErr<Obj> 
    {
        // assert matching lengths of params and args
        (self.len() != args.len()).into_result(UnmatchedParamLists)?;

        // store previous argument values
        let prev = self.try_map_collect(|obj| { env.eval(obj.deref()) })?;

        // apply passed argument values
        for i in 0..self.len() {
            // store evaluation and set to prevent
            // borrow issues where the evaluations
            // needs to borrow a mutably borrowed value
            let res = env.eval(args.get(i)?.deref())?;
            self.get_mut(i)?.set(&res);
        }

        let res = f();

        // reset argument values to previous
        for i in 0..self.len() {
            self.get_mut(i)?.set(prev.get(i)?.deref());
        }

        res
    }

    pub fn macro_scope<F>(&self, env: &Env, args: &mut NodeIter, mut f: F) -> JtsErr<Obj>
    where F: FnMut() -> JtsErr<Obj> 
{
    // assert matching lengths of params and args
    (self.len() != args.len()).into_result(UnmatchedParamLists)?;

    // store previous argument values
    let prev = self.try_map_collect(|obj| { env.eval(obj.deref()) })?;

    // apply passed argument values
    for i in 0..self.len() {
        // store evaluation and set to prevent
        // borrow issues where the evaluations
        // needs to borrow a mutably borrowed value
        let res = args.get_shared(i)?;
        self.get_mut(i)?.set(&res.into_obj());
    }

    let res = f();

    // reset argument values to previous
    for i in 0..self.len() {
        self.get_mut(i)?.set(prev.get(i)?.deref());
    }

    res
}

    /// creates a lexical scope for self's elements where self 
    /// provides both the parameters and the arguments. The closure 
    /// is executed in respect to the scope, the parameters are resets, 
    /// and the evaluation of the execution is returned
    /// 
    /// ## Example
    /// 
    /// ```
    /// (let
    ///     ( (a 1)
    ///       (b 2) )
    /// 
    ///     (+ a b))
    /// ```
    /// 
    /// - create a scope where a = nil and b = nil
    /// - set a = 1 and b = 2 (provided by self)
    /// - evaluate (+ a b) => (+ 1 2)
    /// - return 3 and reset a = nil and b = nil
    
    pub fn anonymous_scope<F>(&self, env: &Env, mut f: F) -> JtsErr<Obj>
        where F: FnMut() -> JtsErr<Obj> 
    {
        // store previous argument values
        let prev = self.try_map_collect(|obj| {
            match obj.deref() {
                Obj::List(node) => Ok(node.get(0)?.clone()),
                _ => Ok(obj.clone())
            }
        })?;

        // apply passed argument values
        for obj in self.args.iter() {
            match obj.borrow_mut().deref() {
                Obj::List(node) => node.get_mut(0)?.set(&env.eval(node.get(1)?.deref())?),
                _ => obj.borrow_mut().set_to(())
            }    
        }

        let res = f();

        // reset argument values to previous
        for (obj, prev) in self.args.iter().zip(prev.into_iter()) {
            match obj.borrow_mut().deref() {
                Obj::List(node) => node.get_mut(0)?.set(prev.deref()),
                _ => obj.borrow_mut().set(prev.deref())
            }    
        }

        res
    }

    /// executes the closure for each element and 
    /// returns the result of the last element's evaluation
    pub fn progn<F>(&self, mut f: F) -> JtsErr<Obj>  
        where F: FnMut(Ref<'_, Obj>) -> JtsErr<Obj> 
    {
        let bounds = self.args.len().checked_sub(self.offset + 1).unwrap_or(0);

        for i in 0..bounds {
            f(self.get(i)?)?;
        }

        f(self.get(bounds)?)
    }

    /// executes the closure 'f1' for each element and returns
    /// the result of the last element's evaluation in respect to 'f2'
    pub fn progn_then<F1, F2>(&self, mut f1: F1, mut f2: F2) -> JtsErr<Obj>  
        where F1: FnMut(Ref<'_, Obj>) -> JtsErr, F2: FnMut(Ref<'_, Obj>) -> JtsErr<Obj> 
    {
        let bounds = self.args.len().checked_sub(self.offset + 1).unwrap_or(0);

        for i in 0..bounds {
            f1(self.get(i)?)?;
        }

        f2(self.get(bounds)?)
    }

    /// collects elements of the iterator mapped by
    /// a closure returning a Result. If an error is
    /// ever found, the collection ends and the error
    /// propogated
    pub fn try_map_collect<F>(&self, mut f: F) -> JtsErr<Node> 
        where F: FnMut(Ref<'_, Obj>) -> JtsErr<Obj>
    {
        let mut err = Ok(());
        let args = self.scan(&mut err, |e, obj| {
                match f(obj) {
                    Ok(obj) => Some(new_shared(obj)),

                    Err(err) => {
                        **e = Err(err);
                        None
                    }
                }
            })
            .collect();
        
        err?;
        Ok(Node {args})
    } 

    // TODO: write description
    pub fn try_map_collect_shared<F>(&self, mut f: F) -> JtsErr<Node> 
    where F: FnMut(&Shared<Obj>) -> JtsErr<Shared<Obj>>
    {
        let mut err = Ok(());
        let args = self.args.iter().scan(&mut err, |e, obj| {
                match f(obj) {
                    Ok(obj) => Some(obj),

                    Err(err) => {
                        **e = Err(err);
                        None
                    }
                }
            })
            .collect();
        
        err?;
        Ok(Node {args})
    } 
}