use crate::lex;

pub fn eval(x: &lex::Exp) -> Result<lex::Exp, ()> {
    match x {
        lex::Exp::Atom(lex::Atom::Symbol(ref s)) => match s.as_str() {
            "if" => eval_conditional(x),
            "define" => eval_define(x),
            _ => eval_proc(x),
        },
        lex::Exp::Atom(lex::Atom::Number(_)) => Ok(x.clone()),
        l => eval_proc(l),
    }
}
fn eval_conditional(x: &lex::Exp) -> Result<lex::Exp, ()> {
    Ok(x.clone())
}
fn eval_define(x: &lex::Exp) -> Result<lex::Exp, ()> {
    Ok(x.clone())
}
fn eval_proc(x: &lex::Exp) -> Result<lex::Exp, ()> {
    match x {
        lex::Exp::List(ref l) => {
            let proc = eval(&l[0]).unwrap();
            let args = &l[1..];
            for a in args {
                let _ = eval(&a);
            }
            proc.eval(args)
        }
        _ => Ok::<lex::Exp, ()>(x.clone()),
    }
}
