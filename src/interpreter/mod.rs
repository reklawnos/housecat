use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::fmt::Display;
use std::collections::HashMap;
use lexer::Lexer;
use evaluator::value::Value;
use evaluator::evaluate;
use ast::{Stmt};
use parser;

fn interpreter_failure<T, D: Display>(message: D) -> Result<T, String> {
    Err(format!("INTERPRETER FAILURE: {}", message))
}

pub struct Interpreter<'a> {
    lexer: Lexer<'a>,
    statements: Vec<Stmt<'a>>,
    defs: HashMap<Value, Value>
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Interpreter<'a> {
        Interpreter {
            lexer: Lexer::new(),
            statements: Vec::with_capacity(100),
            defs: HashMap::with_capacity(100)
        }
    }

    pub fn interpret_file(&'a mut self, filename: &str) -> Result<&mut HashMap<Value, Value>, String> {
        let path = &Path::new(filename);
        let mut file = match File::open(path) {
            Err(err) => {
                {return interpreter_failure(format!("couldn't open {}: {}", path.display(), err));}
            },
            Ok(file) => file,
        };
        let mut file_string = String::new();
        match file.read_to_string(&mut file_string) {
            Err(err) => {return interpreter_failure(format!("couldn't read {}: {}", path.display(), err));},
            Ok(_) => ()
        }
        let lex_result = self.lexer.lex(file_string);
        let ast = match lex_result {
            Err(s) => {return Err(s);}
            Ok(toks) => {
                let parse_result = parser::parse_tokens(&toks[..], &mut self.statements);
                match parse_result {
                    Ok(v) => v,
                    Err(s) => {return interpreter_failure(format!("failed to parse: {}", s));}
                }
            }
        };
        match evaluate(&ast, &mut self.defs) {
            Ok(_) => Ok(&mut self.defs),
            Err(e) => Err(e)
        }
    }
}
