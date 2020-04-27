use json::{self, JsonValue};
use std::fs::File;
use std::env;

struct program {
    items : Vec<String>
}

fn read_json(filename : &String) -> String {
    let content = match std::fs::read_to_string(filename) {
        Ok(r) => r,
        Err(_) => panic!("Error, impossible to open the file")
    };
    content
}



fn handle_rules (rules : &JsonValue) -> String {

}

fn handle_assignment_statements(assignements : &JsonValue) -> String {
    String::new()
}

fn handle_expression_statements(expressions : &JsonValue) -> String {
    String::new()
}

fn handle_name (name : &JsonValue) -> String {

}

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("You should pass one argument!!!");
    }
    let filename = &args[1];
    let file_content = read_json(&filename);
    let grammar = match json::parse(&file_content) {
        Ok(g) => g,
        Err(_) => panic!("Impossible to parse json")
    };

    match grammar {
        JsonValue::Object(o) => {
            let name : &JsonValue;
            match o.get("name") {
                Some(a) => name = a,
                None => panic!("Error missing name"),
            };
            let mut final_string = handle_name(&name);
        },
        _ => println!("Some other value")
    };

    println!("Hello, world!");
}
