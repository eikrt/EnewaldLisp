use crate::lex;
use crate::plus;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Operator {
    Plus,
}
pub struct Environment {
    pub operators: HashMap<Operator, Box<dyn Fn(&[lex::Exp]) -> i64 + Send + Sync>>,
}
fn map_procs(p: &lex::Atom) -> Operator {
    match p {
        lex::Atom::Symbol(t) => match t.as_str() {
            "+" => Operator::Plus,
            &_ => todo!(),
        },
        &_ => todo!(),
    }
}

impl Default for Environment {
    fn default() -> Self {
        let mut operators: HashMap<Operator, Box<dyn Fn(&[lex::Exp]) -> i64 + Send + Sync>> =
            HashMap::new();

        operators.insert(Operator::Plus, Box::new(|slice| plus!(slice)));

        Environment { operators }
    }
}
impl Environment {
    pub fn eval(&self, a: &lex::Atom, args: &[lex::Exp]) -> Result<lex::Exp, ()> {
        let atom = lex::Exp::Atom(lex::Atom::Number(self
            .operators
            .get(&map_procs(a))
            .unwrap()(&args[0..2])));
        Ok(atom)
    }
}
lazy_static! {
    pub static ref ENV: Mutex<Environment> = Mutex::new(Environment::default());
}
