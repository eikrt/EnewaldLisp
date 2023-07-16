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
            Exp::Atom(a) => ENV.lock().unwrap().eval(a, l),
            a => Ok(a.clone()),
        }
    }
    pub fn print(&self) {
        match self {
            Exp::Atom(Atom::Number(a)) => {
                println!("{}", a);
            }
            &_ => todo!(),
        }
    }
}
