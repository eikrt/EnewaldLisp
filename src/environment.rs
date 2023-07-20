use crate::lex::{Atom, Exp};
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
    pub definitions: HashMap<Definition, Box<dyn Fn(&[Exp]) -> Exp + Send + Sync>>,
}
fn map_procs(p: &Atom) -> Definition {
    match p {
        Atom::Symbol(t) => match t.as_str() {
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
        let mut definitions: HashMap<Definition, Box<dyn Fn(&[Exp]) -> Exp + Send + Sync>> =
            HashMap::new();

        definitions.insert(Definition::Plus, Box::new(|slice| plus!(slice)));
        definitions.insert(Definition::Minus, Box::new(|slice| minus!(slice)));
        definitions.insert(Definition::Multi, Box::new(|slice| multi!(slice)));
        definitions.insert(Definition::Div, Box::new(|slice| div!(slice)));
        definitions.insert(Definition::Modulo, Box::new(|slice| modulo!(slice)));
        definitions.insert(
            Definition::Command,
            Box::new(|slice| {
                let mut svec: Vec<String> = vec![];
                let mut cmd = "".to_string();
                'it: for c in slice.iter() {
                    match c {
                        Exp::Atom(a) => match a {
                            Atom::Symbol(s) => {
                                match s.chars().nth(0).unwrap() {
                                    '%' => {
                                        cmd.push_str(format!(" {} ", &s[1..]).as_str());
                                        continue 'it;
                                    }
                                    _ => {}
                                };
                                match s.chars().nth(s.len() - 1).unwrap() {
                                    '%' => {
                                        cmd.push_str(format!(" {} ", &s[..s.len() - 1]).as_str());
                                    }
                                    _ => {
                                        if !cmd.is_empty() {
                                            cmd.push_str(format!(" {}", &s).as_str())
                                        } else {
                                            svec.push(s.to_string());
                                        }
                                    }
                                };
                            }
                            Atom::Number(n) => {
                                let string = n.to_string();
                                svec.push(string)
                            }
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                svec.push(cmd);
                let stdout = process::Command::new(&svec[0])
                    .args(&svec[1..])
                    .output()
                    .expect("Failed executing command");
                Exp::Atom(Atom::Symbol(
                    std::str::from_utf8(&stdout.stdout).unwrap().to_string(),
                ))
            }),
        );

        Environment { definitions }
    }
}
impl Environment {
    pub fn eval(&self, a: &Atom, args: &[Exp]) -> Result<Exp, ()> {
        let val = match args {
            [] => Ok(Exp::Atom(a.clone())),
            _ => Ok(self.definitions.get(&map_procs(a)).unwrap()(&args)),
        };

        val
    }
}
lazy_static! {
    pub static ref ENV: Mutex<Environment> = Mutex::new(Environment::default());
}
