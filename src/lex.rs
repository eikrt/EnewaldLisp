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
    pub fn eval_proc(&self) -> Result<Exp, ()> {
        #[allow(unused_assignments)]
        let mut proc = Exp::Atom(Atom::Number(0));
        #[allow(unused_assignments)]
        let mut args = Exp::List(vec![]);
        proc = match self {
            Exp::Atom(_) => self.clone(),
            Exp::List(l) => l[0].eval().unwrap().clone(),
        };
        args = match self {
            Exp::Atom(_) => self.clone(),
            Exp::List(l) => Exp::List(l[1..].iter().map(|x| x.eval().unwrap()).collect()),
        };
        ENV.lock()
            .unwrap()
            .eval(&proc.as_atom(), args.as_vec().as_slice())
    }
    fn as_vec(&self) -> Vec<Exp> {
        match self {
            Exp::List(l) => l.clone(),
            Exp::Atom(_) => panic!("Bug"),
        }
    }
    fn as_atom(&self) -> Atom {
        match self {
            Exp::List(_) => panic!("Bug"),
            Exp::Atom(a) => a.clone(),
        }
    }
    pub fn eval(&self) -> Result<Exp, ()> {
        match self {
            Exp::Atom(_) => Ok(self.clone()),
            Exp::List(_) => self.eval_proc(),
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
