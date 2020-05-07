use crate::grammar::{self, RuleBody};

enum Pos {
    Value(usize),
    None,
}

struct Name (String);

impl Name {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

enum Node {
    Intermediate(u32),
    IntermediateChoice(u32, Option<Box<Vec<Node>>>),
    IntermediateSeq(Option<Box<Vec<Node>>>),
    Leaf(Name),
}

struct Statement (Name, Node);

impl Statement {
    fn get_name (&self) -> String {
        self.0.to_string()
    }

    fn get_node (&self) -> &Node {
        &self.1
    }
}

pub struct Ast (Name, Vec<Statement>);

impl Ast {
    fn _get_name (&self) -> String {
        self.0.to_string()
    }

    fn get_statements (&self) -> &Vec<Statement> {
        &self.1
    }
}

fn expand_rulebody (rule : &RuleBody, new_statements : &mut Vec<Statement>, id_int: &mut u32, id_choice: &mut u32) -> Node {
    match rule {
        RuleBody::Repeat(r) => if let Some(inner) = r {
            let node = expand_rulebody(inner, new_statements, id_int, id_choice);
            *id_int = *id_int + 1;
            new_statements.push(Statement(Name(format!("Intermediate{}", id_int)), node));
            return Node::Intermediate(*id_int)
        },

        RuleBody::Choice(c) => if let Some(inner) = c {

            let mut choice_node : Vec<Node> = vec!();
            let curr_id = *id_choice;
            for el in inner.iter() {
                *id_choice = *id_choice + 1;
                choice_node.push(expand_rulebody(&el, new_statements, id_int, id_choice));
            }

            return Node::IntermediateChoice(curr_id, Some(Box::new(choice_node)))
        },

        RuleBody::Seq(s) => if let Some(inner) = s {

            let mut seq_node : Vec<Node> = vec!();

            for el in inner.iter() {
                seq_node.push(expand_rulebody(&el, new_statements, id_int, id_choice));
            }

            return Node::IntermediateSeq(Some(Box::new(seq_node)))
        },

        RuleBody::Symbol(s)
            | RuleBody::Str(s)
            | RuleBody::Pattern(s) =>
            if let Some(inner) = s {
                return Node::Leaf(Name(inner.to_string()))
            },
    };
    panic!("Error, It should never arrive here")
}

fn is_intermediate_used(name : &String, n : &Node) -> bool{
    match n {
        Node::Intermediate(c) => {
            if *name == format!("Intermediate{}", c) {
                true
            }
            else {
                false
            }
        }
        Node::IntermediateChoice(_, c) |
            Node::IntermediateSeq(c) => {
                let nodes = match c {
                    Some(c) => c,
                    None => panic!("Empty intermediate node found")
                };
                for node in nodes.iter() {
                    if is_intermediate_used(name, node) {
                        return true;
                    }
                }
                false
        }
        _ => false
    }
}

fn get_pos_statement(name : String, st : &Vec<Statement>) -> Pos {
    for (pos, el) in st.iter().enumerate() {
        if is_intermediate_used(&name , el.get_node()) {
                return Pos::Value(pos+1);
        }
    }
    Pos::None
}

pub fn convert_to_ast (grammar : grammar::Grammar) -> Ast {
    let id = grammar.get_ident();

    let rules = grammar.get_rules();

    let mut ast : Vec<Statement> = vec!();
    let mut new_statements : Vec<Statement> = vec!();

    let mut id_int = 0;
    let mut id_cho = 0;

    for rule in rules.get_list() {
        let node = expand_rulebody(rule.get_rulebody(), &mut new_statements, &mut id_int, &mut id_cho);
        ast.push(Statement(Name(rule.get_ident().get_string().to_string()), node));
    }

    // Place new intermediate statements in the right place in the list
    while !new_statements.is_empty() {
        let el = new_statements.pop().unwrap();
        match get_pos_statement(el.get_name(), &ast) {
            Pos::Value(x) =>
                ast.insert(x as usize, el),

            Pos::None => ()
        }
    }

    Ast(Name(id.get_string().to_string()), ast)
}

fn node_to_string(node : &Node) -> String {
    let mut output : String = String::new();

    match node {
        Node::Intermediate(i) => {
            format!("{}list(Intermediate{}) ",output, i)
        },

        Node::IntermediateChoice(i, c) => {
            let choices = match c {
                Some(c) => c,
                None => panic!("Empty intermediate choice node found")
            };
            let mut choice_counter = 0;
            for choice in choices.iter() {
                output = format!("{}\n\t| Intermediate_type{}(", output, i+choice_counter);
                choice_counter +=1;
                output = format!("{}{}", output, node_to_string(choice));
                output = format!("{})", output);
            }
            output
        },

        Node::IntermediateSeq(s) => {
            let seq = match s {
                Some (c) => c,
                None => panic!("Empty intermediate seq node found")
            };

            output = format!("{}(", output);
            let mut comma_counter = 0;
            for sym in seq.iter() {
                comma_counter += 1;
                output = format!("{}{}", output, node_to_string(sym));
                if comma_counter != seq.len() {
                    output = format!("{}, ", output);
                }
            }
            format!("{})", output)
        },

        Node::Leaf(n) =>
            format!("{}{}", output, n.to_string())
    }
}

fn statement_to_string(statement : &Statement) -> String {
    let output : String;

    output = format!("{} = ", statement.get_name());

    let node = statement.get_node();

    format!("{}{}\n", output, node_to_string(node))
}

pub fn ast_to_string (ast : Ast) -> String {
    let mut output : String = String::new();
    let mut and : String = String::new();

    for el in ast.get_statements() {
        output = format!("{}\n{}{}", output, and, statement_to_string(el));
        and = String::from("and ");
    }
    output = output.trim().to_string();
    format!("type {};", output)
}

