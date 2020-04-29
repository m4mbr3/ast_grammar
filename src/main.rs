use json::{self};
use std::env;

mod grammar;
mod converter;

use grammar::parse_grammar;
use converter::convert_to_ast;
use converter::ast_to_string;

// Read the json in memory to be parse - NOT OK for insanely large json files
fn read_json(filename : &String) -> String {
    let content = match std::fs::read_to_string(filename) {
        Ok(r) => r,
        Err(_) => panic!("Error, impossible to open the file")
    };
    content
}

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("You should pass one argument!!!");
    }

    let filename = &args[1];

    let file_content = read_json(&filename);

    // load grammar in json format
    let grammar_json = match json::parse(&file_content) {
        Ok(g) => g,
        Err(_) => panic!("Impossible to parse json")
    };

    let g = parse_grammar(&grammar_json);

    let ast = convert_to_ast(g);

    let definition = ast_to_string(ast);
    println!("{}", definition);
}
