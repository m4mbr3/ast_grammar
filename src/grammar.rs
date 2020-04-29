use json::{JsonValue};

pub struct Ident(String);

impl Ident {
    pub fn get_string(&self) -> &String {
        return &self.0
    }
}

pub enum RuleBody {
    Repeat(Option<Box<RuleBody>>),
    Choice(Option<Box<Vec<RuleBody>>>),
    Seq(Option<Box<Vec<RuleBody>>>),
    Symbol(Option<String>),
    Str(Option<String>),
    Pattern(Option<String>),
}
pub struct Rule (Ident, RuleBody);

impl Rule {
    pub fn get_ident (&self) ->  &Ident {
        return &self.0
    }

    pub fn get_rulebody (&self) -> &RuleBody {
        return &self.1
    }
}

pub struct Rules (Vec<Rule>);

impl Rules {
    pub fn get_list (&self) -> &Vec<Rule> {
        return &self.0
    }
}

pub struct Grammar (Ident, Rules);

impl Grammar {
    pub fn get_ident (&self) -> &Ident {
        return &self.0
    }

    pub fn get_rules (&self) -> &Rules {
        return &self.1
    }
}

fn handle_members (members: &JsonValue) -> Vec<RuleBody> {
    let mut res : Vec<RuleBody> = vec!();

    for obj in members.members() {
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
                    return RuleBody::Str(Some(format!("string /* {} */", name.to_string())));
                },

                "\"PATTERN\"" => {
                    let _name = match obj.get("value") {
                        Some(n) => n,
                        None => panic!("Value field not found")
                    };
                    return RuleBody::Pattern(Some(format!("string ")));
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
    panic!("Error, it should never arrive here {}", body)
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

    Rules(rls)
}

pub fn parse_grammar(grammar: &JsonValue) -> Grammar {
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
            return Grammar(Ident(name), handle_rules(rules));


        },
        _ => panic!("Invalid object passed")
    };
}
