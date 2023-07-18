use crate::lex;
use crate::{div, minus, modulo, multi, plus};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::process;
use std::sync::Mutex;
#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Definition {
    Plus,
    Minus,
    Multi,
    Div,
    Modulo,
    Command,
}
pub struct Environment {
    pub definitions: HashMap<Definition, Box<dyn Fn(&[lex::Exp]) -> lex::Exp + Send + Sync>>,
}
fn map_procs(p: &lex::Atom) -> Definition {
    match p {
        lex::Atom::Symbol(t) => match t.as_str() {
            "+" => Definition::Plus,
            "-" => Definition::Minus,
            "*" => Definition::Multi,
            "/" => Definition::Div,
            "%" => Definition::Modulo,
            "command" => Definition::Command,
            &_ => todo!(),
        },
        &_ => todo!(),
    }
}

impl Default for Environment {
    fn default() -> Self {
        let mut definitions: HashMap<
            Definition,
            Box<dyn Fn(&[lex::Exp]) -> lex::Exp + Send + Sync>,
        > = HashMap::new();

        definitions.insert(Definition::Plus, Box::new(|slice| plus!(slice)));
        definitions.insert(Definition::Minus, Box::new(|slice| minus!(slice)));
        definitions.insert(Definition::Multi, Box::new(|slice| multi!(slice)));
        definitions.insert(Definition::Div, Box::new(|slice| div!(slice)));
        definitions.insert(Definition::Modulo, Box::new(|slice| modulo!(slice)));
        definitions.insert(
            Definition::Command,
            Box::new(|slice| {
                let mut svec: Vec<String> = vec![];
                let command = match slice.iter().nth(0).unwrap() {
                    lex::Exp::Atom(a) => match a {
                        lex::Atom::Symbol(s) => s,
                        _ => todo!(),
                    },
                    _ => todo!(),
                };
                for c in slice.iter().skip(1) {
                    match c {
                        lex::Exp::Atom(a) => match a {
                            lex::Atom::Symbol(s) => svec.push(s.to_string()),
                            lex::Atom::Number(n) => {
                                let string = n.to_string();
                                svec.push(string)
                            }
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                let stdout = process::Command::new(command)
                    .args(svec)
                    .output()
                    .expect("Failed executing command");
                lex::Exp::Atom(lex::Atom::Symbol(
                    std::str::from_utf8(&stdout.stdout).unwrap().to_string(),
                ))
            }),
        );

        Environment { definitions }
    }
}
impl Environment {
    pub fn eval(&self, a: &lex::Atom, args: &[lex::Exp]) -> Result<lex::Exp, ()> {
        let val = match args {
            [] => Ok(lex::Exp::Atom(a.clone())),
            _ => Ok(self.definitions.get(&map_procs(a)).unwrap()(&args[0..])),
        };

        val
    }
}
lazy_static! {
    pub static ref ENV: Mutex<Environment> = Mutex::new(Environment::default());
}
