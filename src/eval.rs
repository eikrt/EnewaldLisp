use crate::lex;

pub fn eval(x: &lex::Exp) -> Result<lex::Exp, ()> {
    match x {
        lex::Exp::Atom(lex::Atom::Symbol(ref s)) => match s.as_str() {
            _ => Ok(x.clone()),
        },
        lex::Exp::Atom(lex::Atom::Number(_)) => Ok(x.clone()),
        lex::Exp::Atom(lex::Atom::Float(_)) => Ok(x.clone()),
        lex::Exp::List(_) => eval_proc(x),
    }
}
fn eval_proc(x: &lex::Exp) -> Result<lex::Exp, ()> {
    match x {
        lex::Exp::List(ref l) => {
            let proc = eval(&l[0]).unwrap();
            let args = &l[1..];
            let mut new_args = Vec::new();
            for a in args {
                let n = eval(&a).unwrap();
                new_args.push(n);
            }
            proc.eval(&new_args[..])
        }
        _ => Ok::<lex::Exp, ()>(x.clone()),
    }
}
