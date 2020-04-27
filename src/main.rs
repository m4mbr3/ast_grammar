use json::{self, JsonValue};
use std::env;

struct Ident(String);

enum RuleBody {
    Repeat(Option<Box<RuleBody>>),
    Choice(Option<Box<Vec<RuleBody>>>),
    Seq(Option<Box<Vec<RuleBody>>>),
    Symbol(Option<String>),
    Str(Option<String>),
    Pattern(Option<String>),
}

struct Rule (Ident, RuleBody);

struct Rules (Vec<Rule>);

struct Grammar (Ident, Rules);

// Read the json in memory to be parse - NOT OK for insanely large json files
fn read_json(filename : &String) -> String {
    let content = match std::fs::read_to_string(filename) {
        Ok(r) => r,
        Err(_) => panic!("Error, impossible to open the file")
    };
    content
}
fn handle_members (members: &JsonValue) -> Vec<RuleBody> {
    let mut res : Vec<RuleBody> = vec!();

    println!("members: {}", members);

    for (_, obj) in members.entries() {
        res.push(handle_rulebody(obj));
    }

    res
}

fn handle_rulebody (body : &JsonValue) -> RuleBody {
    if !body.is_object() {
        panic!("handle_rulebody did not receive a json object");
    }

    if let JsonValue::Object(obj) = body {
        let pattern : String;
        if let Some(value) = obj.get("type") {
            pattern = value.dump();
            match pattern.as_ref() {
                "\"REPEAT\"" => {
                    let content = match obj.get("content") {
                        Some(c) => c,
                        None => panic!("Content field not found")
                    };
                    return RuleBody::Repeat(Some(Box::new(handle_rulebody(content))))
                },

                "\"CHOICE\"" => {
                    let members = match obj.get("members") {
                        Some(m) => m,
                        None => panic!("Memers field not found")
                    };
                    return RuleBody::Choice(Some(Box::new(handle_members(members))))
                },

                "\"SEQ\"" => {
                    let members = match obj.get("members") {
                        Some(m) => m,
                        None => panic!("Members field not found")
                    };
                    return RuleBody::Seq(Some(Box::new(handle_members(members))))
                },

                "\"SYMBOL\"" => {
                    let name = match obj.get("name") {
                        Some(n) => n,
                        None => panic!("Name field not found")
                    };
                    return RuleBody::Symbol(Some(name.to_string()));
                },

                "\"STRING\"" => {
                    let name = match obj.get("value") {
                        Some(n) => n,
                        None => panic!("Value field not found")
                    };
                    return RuleBody::Str(Some(name.to_string()));
                },

                "\"PATTERN\"" => {
                    let name = match obj.get("value") {
                        Some(n) => n,
                        None => panic!("Value field not found")
                    };
                    return RuleBody::Pattern(Some(name.to_string()));
                },

                "\"PREC\"" | "\"PREC_LEFT\"" | "\"PREC_RIGHT\"" | "\"PREC_DYNAMIC\"" => {
                    let content = match obj.get("content") {
                        Some(c) => c,
                        None => panic!("Value field not found")
                    };
                    return handle_rulebody(content)
                },

                "\"FIELD\"" => {
                    let name = match obj.get("name") {
                        Some(n) => n,
                        None => panic!("Value field not found")
                    };
                    return handle_rulebody(name)
                },

                _ => panic!("Type field not found {}", body)
            }
        }
    }

    RuleBody::Str(None)
}

fn handle_rule (keyname: String, rule : &JsonValue) -> Rule {
    if !rule.is_object() {
        panic!("handle_rule did not receive a json object");
    }

    Rule(Ident(keyname), handle_rulebody(rule))
}

fn handle_rules(rules : &JsonValue) -> Rules {
    let mut rls : Vec<Rule> = vec!();

    if !rules.is_object() {
        panic!("handle_rules did not receive a json object");
    }

    for (key,obj) in rules.entries() {
        rls.push(handle_rule(key.to_string(), obj))
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
