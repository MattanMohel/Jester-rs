use std::ops::Deref;

use crate::core::{
    env::Env,
    err::{
        JtsErr,
        JtsErrType::*
    },
    objects::Obj, 
    functions::{
        FnNative, 
        FnMacro
    }, 
    types::TypeId 
};

impl Env {
    pub fn std_lib(&mut self) -> JtsErr {
        //@decl T
        //@doc
        // the constant boolean value 'True'
        self.add_symbol("T", Obj::new_const(true))?;

        //@decl F
        //@doc 
        // the constant boolean value 'False'
        self.add_symbol("F", Obj::new_const(false))?;

        //@dec const pi
        //@doc the constant f64 value 'PI,' equal to 3.14159265
        self.add_symbol("pi", Obj::new_const::<f64>(3.1415926535))?;

        //@decl set
        //@params
        // ('set setter)
        //@return
        // the value of setter
        //@doc
        // sets 'set' to the value of 'setter'
        self.add_symbol("set", Obj::new_bridge(|env, node| {
            let setter = env.eval(node.get(1)?.deref())?;
            node.get_mut(0)?.set(&setter);
            Ok(setter)
        }))?;

        //@decl defun
        //@params
        // ('func '(params..) 'body)
        //@return
        // the defined function symbol
        //@doc
        // creates a native function named 'symbol' which recieves
        // 'params' as arguments and outputs the evaluation of 'body'
        //@example
        // (defun add (a b)
        //     (+ a b))
        //
        // (assert-eq (add 10 20) 30)
        self.add_symbol("defun", Obj::new_bridge(|_, node| {
            let native = FnNative {
                params: node.get(1)?.is_node()?.clone(),
                body: node.into_node_from(2)
            };

            node.get_mut(0)?.set_to(native);
            Ok(node.get(0)?.clone())
        }))?;

        self.add_symbol("macro", Obj::new_bridge(|_, node| {
            let fn_macro = FnMacro {
                params: node.get(1)?.is_node()?.clone(),
                body: node.into_node_from(2)
            };

            node.get_mut(0)?.set_to(fn_macro);
            Ok(node.get(0)?.clone())
        }))?;

        //@decl let
        //@params
        // ('func '( (args values).. ) 'body)
        //@return 
        // the evaluation of 'body'
        //@doc 
        // creates a lexical scope bounding 'args' to their
        // respective 'values'. 'body' is then evaluated in respect
        // to 'args' and returned. Following its execution, all 'args'
        // are reverted to their previous values
        //@example 
        // (set a 1)
        // (set b 2)
        // (set c 
        //     (let (( a 10 )
        //           ( b 20 ))
        // 
        //         (+ a b)))
        // 
        // (assert-eq a 1)
        // (assert-eq b 2)
        // (assert-eq c 30)
        self.add_symbol("let", Obj::new_bridge(|env, node| {    
            let args = node.shift()?;
            let args = args.deref().borrow();
            let args = args.is_node()?.into_iter();

            args.anonymous_scope(env, || node.progn(|obj| env.eval(obj.deref()) ) )
        }))?;

        //@decl do
        //@params
        //('body...)
        //@return
        // the evaluation of 'body'
        //@doc
        // takes variable amounts of expressions, evluates them, and returns 
        // the evaluation of the final expression
        //@example
        // (set c (do
        //            (set a 10)
        //            (set b 20)
        //            (+ a b)))
        //
        // (assert-eq c 30)
        self.add_symbol("do", Obj::new_bridge(|env, node| {    
            node.progn(|obj| { env.eval(obj.deref()) })
        }))?;

        //@decl =
        //@params
        // (a b)
        //@return
        // the boolean comparison of 'a' and 'b'
        //@doc
        // compares 'a' and 'b' for equality
        self.add_symbol("=", Obj::new_bridge(|env, node| {
            let res = Obj::eq(&env.eval(node.get(0)?.deref())?, &env.eval(node.get(1)?.deref())?)?;
            Ok(Obj::new_const(res))
        }))?;

        //@decl <
        //@params
        // (a b)
        //@return
        // the boolean comparison of 'a' and 'b'
        //@doc
        // compares if 'a' is less than 'b'
        self.add_symbol("<", Obj::new_bridge(|env, node| {
            let res = Obj::le(&env.eval(node.get(0)?.deref())?, &env.eval(node.get(1)?.deref())?)?;
            Ok(Obj::new_const(res))
        }))?;

        //@decl <=
        //@params
        // (a b)
        //@return
        // the boolean comparison of 'a' and 'b'
        //@doc
        // compares if 'a' is less than or equal to 'b'
        self.add_symbol("<=", Obj::new_bridge(|env, node| {
            let res = Obj::le_eq(&env.eval(node.get(0)?.deref())?, &env.eval(node.get(1)?.deref())?)?;
            Ok(Obj::new_const(res))
        }))?;

        //@decl >
        //@params
        // (a b)
        //@return
        // the boolean comparison of 'a' and 'b'
        //@doc
        // compares if 'a' is greater than 'b'
        self.add_symbol(">", Obj::new_bridge(|env, node| {
            let res = !Obj::le_eq(&env.eval(node.get(0)?.deref())?, &env.eval(node.get(1)?.deref())?)?;
            Ok(Obj::new_const(res))
        }))?;

        //@decl <
        //@params
        // (a b)
        //@return
        // the boolean comparison of 'a' and 'b'
        //@doc
        // compares if 'a' is greater than or equal to 'b'
        self.add_symbol(">=", Obj::new_bridge(|env, node| {
            let res = !Obj::le(&env.eval(node.get(0)?.deref())?, &env.eval(node.get(1)?.deref())?)?;
            Ok(Obj::new_const(res))
        }))?;

        //@decl assert
        //@params
        // (a)
        //@return
        // true is 'a' is true, or error
        //@doc
        // asserts 'a' is equal to True
        self.add_symbol("assert", Obj::new_bridge(|env, node| {
            let res = Obj::eq(&env.eval(node.get(0)?.deref())?, &env.eval(node.get(1)?.deref())?)?;

            if res {
                Ok(Obj::Bool(true))
            } else {
                Err(RuntimeAssert)
            }
        }))?;

        //@decl assert-eq
        //@params
        // (a b)
        //@return
        // true is 'a' and 'b' are equal, or error
        //@doc
        // asserts equality of 'a' and 'b'
        self.add_symbol("assert-eq", Obj::new_bridge(|env, node| {
            let res = Obj::eq(&env.eval(node.get(0)?.deref())?, &env.eval(node.get(1)?.deref())?)?;

            if res {
                Ok(true.into_obj())
            } else {
                Err(RuntimeAssert)
            }
        }))?;

        //@decl loop
        //@params
        // ('cond 'body..)
        //@return
        // the final evaluation of body
        //@doc
        // loops and evluates 'body' while 'cond' remains true
        //@example
        // (set i 0)
        //
        // (loop (< i 10)
        //     (+= i i))
        //
        // (assert-eq i 10)
        self.add_symbol("loop", Obj::new_bridge(|env, node| {
            let mut res = Obj::Nil();
            let cond = node.get(0)?;

            while *env.eval(cond.deref())?.is_bool()? {
                res = node.progn(|obj| { env.eval(obj.deref()) })?;
            }
            Ok(res)
        }))?;

        //@decl gensym
        //@params
        // (value-init)
        //@return
        // symbol of the generated symbol
        //@doc
        // generates a garaunteed-unique symbol and initializes 
        // its value to t0 'value-init'
        self.add_symbol("gensym", Obj::new_bridge(|env, node| unsafe {
            println!("gen");
            let val = env.eval(node.get(0)?.deref())?;
            Ok(env.generate_symbol(val)?.into_obj())
        }))?;

        //@decl quote
        //@params
        // (object)
        //@return
        // the quoted symbol of 'object'
        self.add_symbol("quote", Obj::new_bridge(|_, node| {
            Ok(node.get_shared(0)?.into_obj())
        }))?;  
        
        //@decl eval
        //@params
        // (to-eval)
        //@return
        // the evaluated form of 'to-eval'
        //@example
        // (set a
        //     (eval '(+ 10 20)))
        //
        // (assert-eq a 30)
        self.add_symbol("eval", Obj::new_bridge(|env, node| {     
            env.eval_shared(node.get(0)?.deref())
        }))?;

        //@decl if
        //@params
        // (cond 'if-true ( 'if-false nil ))
        //@return
        // the evaluation of 'if-true' if 'cond' is True or the  
        // evaluation of 'if-false' otherwise
        self.add_symbol("if", Obj::new_bridge(|env, node| {
            let cond = *env.eval(node.get(0)?.deref())?.is_bool()?;
            if cond {
                env.eval(node.get(1)?.deref())
            } else {
                match node.get(2) {
                    Ok(if_false) => env.eval(if_false.deref()),
                    Err(_) => Ok(Obj::Nil())
                }
            }
        }))?;
        Ok(())
    }
}