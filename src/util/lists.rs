
use crate::core::objects::{ Obj, Node };
use crate::core::env::Env;

pub fn for_each(env: &mut Env, list: &mut Node, map: fn(&mut Env, &mut Obj)) {
    map(env, list.val());

    while list.next().is_some() {
        list.shift();
        map(env, list.val());
    }  
}

pub fn copy(env: &mut Env, list: &mut Node) -> Node {
    todo!()
}