pub mod dpll;
pub mod parser;
use std::{env, process};

use parser::CNF;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let input_path = &args[1];

    let mut cnf = CNF::from_dirac_file(input_path.clone()).unwrap();
    println!("{ }", cnf.dpll())
}
