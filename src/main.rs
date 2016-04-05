extern crate housecat;
use std::env;

use housecat::Interpreter;

fn main() {
    let command_args: Vec<String> = env::args().collect();
    if command_args.len() <= 1 {
        println!("No .hcat file provided!");
    } else {
        let mut interpreter = Interpreter::new();
        match interpreter.interpret_file(&command_args[1][..]) {
            Ok(_) => (),
            Err(s) => println!("{}", s)
        }
    }
}
