use std::{io::{self, Write}, time::{Instant, Duration}};
use super::{err::JtsErr, objects::Obj, env::{Env, PRELUDE} };
use crate::lex::parser::Parser;

pub struct Repl {
    line: usize,
}

impl Repl {
    pub fn new() -> Self {
        Repl {
             line: 0, 
        }
    }

    pub fn run(&mut self, env: &mut Env) -> JtsErr<Obj> {  
        let mut eval = Obj::Nil();
        let mut time = Duration::new(0, 0);

        loop {
            print!("[{}]>> ", self.line);
            io::stdout().flush()?;
            self.line += 1;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim() {
                "--help" => {

                }
                "--quit" => {
                    println!("quit REPL");
                    break;
                },
                "--time" => {
                    println!("completed in: {:?}", time);
                    continue;
                },
                _ => ()
            }

            let body = Parser::from_string(&input.trim().to_string())?.parse_tokens(env, &String::from(PRELUDE))?;

            let start = Instant::now();
            eval = env.run(&body)?;
            time = start.elapsed();

            println!("{}", eval.to_string(env));
        }

        Ok(eval)
    }
}