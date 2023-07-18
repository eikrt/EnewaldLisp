use crate::environment::ENV;
#[derive(Clone, Debug)]
pub enum Atom {
    Number(i64),
    Float(f64),
    Symbol(String),
}
#[derive(Clone, Debug)]
pub enum Exp {
    Atom(Atom),
    List(Vec<Exp>),
}
impl Exp {
    pub fn eval(&self, l: &[Exp]) -> Result<Exp, ()> {
        match self {
            Exp::Atom(a) => match l {
                [] => Ok(self.clone()),
                _ => ENV.lock().unwrap().eval(a, l),
            },
            a => a.eval(l),
        }
    }
    pub fn print(&self) {
        match self {
            Exp::Atom(Atom::Number(a)) => {
                println!("{}", a);
            }
            Exp::Atom(Atom::Float(f)) => {
                println!("{}", f);
            }
            Exp::Atom(Atom::Symbol(s)) => {
                println!("{}", s);
            }
            Exp::List(l) => {
                println!("{:?}", l);
            }
        }
    }
}
