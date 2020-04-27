use json::{self, JsonValue};
use std::env;

struct Ident(String);
enum RuleBody {
    Repeat(Box<RuleBody>),
    Choice(Vec<Box<RuleBody>>),
    Seq(Vec<Box<RuleBody>>),
    Symbol(String),
    Str(String),
    Pattern(String),
}
struct Rule (Ident, RuleBody);
struct Rules (Vec<Rule>);
struct Grammar (Ident, Rules);

fn read_json(filename : &String) -> String {
    let content = match std::fs::read_to_string(filename) {
        Ok(r) => r,
        Err(_) => panic!("Error, impossible to open the file")
    };
    content
}

fn handle_rule (rule : &JsonValue) -> Rule {
    println!("{}", rule.dump());
    Rule(Ident(String::new()), RuleBody::Str(String::new()))
}

fn handle_rules(rules : &JsonValue) -> Rules {
    let mut rls : Vec<Rule> = vec!();

    if !rules.is_object() {
        panic!("handle_rules received not a json object");
    }

    let entries = rules.entries();

    for (i, k) in entries {
        println!("key {}", i);
        rls.push(handle_rule(k))
    }

    Rules(vec!())
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
            let name : String;
            let rules : &JsonValue;

            // Name of the program
            match o.get("name") {
                Some(value) => name = value.dump(),
                None => panic!("Error missing name"),
            };

            // List of rules
            match o.get("rules") {
                Some(value) => rules = value,
                None => panic!("Error missing rules"),
            };
            // Generate the inner representation
            let g = Grammar(Ident(name), handle_rules(rules));
        },
        _ => println!("Some other value")
    };
}
