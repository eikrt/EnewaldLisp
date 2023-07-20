use crate::environment::ENV;
use crate::lex::{Atom, Exp};
use crate::parser;
use crate::test;
use crate::tokenizer;
use crate::{div, minus, modulo, multi, plus};
use std::collections::HashMap;
// unit tests
#[test]
fn tokenize() {
    assert_eq!(
        tokenizer::tokenize("(+ 1 1)"),
        vec!["(", "+", "1", "1", ")"]
    );
}
#[test]
fn tokenize_whitespaces() {
    assert_eq!(
        tokenizer::tokenize(" ( + 1 1 ) "),
        vec!["(", "+", "1", "1", ")"]
    );
}
#[test]
fn parse() {
    assert_eq!(
        Exp::List(vec![Exp::List(vec![
            Exp::Atom(Atom::Symbol("+".to_string())),
            Exp::Atom(Atom::Number(1)),
            Exp::Atom(Atom::Number(1)),
        ])]),
        parser::parse("(+ 1 1)").unwrap()
    );
}
#[test]
fn read_from_tokens() {
    assert_eq!(
        parser::read_from_tokens(&mut vec![
            "(".to_string(),
            "+".to_string(),
            "1".to_string(),
            "1".to_string(),
            ")".to_string()
        ])
        .unwrap(),
        vec![Exp::List(vec![
            Exp::Atom(Atom::Symbol("+".to_string())),
            Exp::Atom(Atom::Number(1)),
            Exp::Atom(Atom::Number(1))
        ])]
    )
}
#[test]
fn atom() {
    assert_eq!(Atom::Symbol("x".to_string()), parser::atom("x"));
    assert_eq!(Atom::Number(7), parser::atom("7"));
    assert_eq!(Atom::Float(7.1), parser::atom("7.1"));
}
#[test]
fn plus() {
    assert_eq!(
        plus!([Exp::Atom(Atom::Number(1)), Exp::Atom(Atom::Number(1))]),
        Exp::Atom(Atom::Number(2))
    );
}
#[test]
fn minus() {
    assert_eq!(
        plus!([Exp::Atom(Atom::Number(1)), Exp::Atom(Atom::Number(1))]),
        Exp::Atom(Atom::Number(2))
    );
}
#[test]
fn multi() {
    assert_eq!(
        multi!([Exp::Atom(Atom::Number(2)), Exp::Atom(Atom::Number(2))]),
        Exp::Atom(Atom::Number(4))
    );
}
#[test]
fn div() {
    assert_eq!(
        div!([Exp::Atom(Atom::Number(4)), Exp::Atom(Atom::Number(2))]),
        Exp::Atom(Atom::Number(2))
    );
}
#[test]
fn modulo() {
    assert_eq!(
        modulo!([Exp::Atom(Atom::Number(5)), Exp::Atom(Atom::Number(2))]),
        Exp::Atom(Atom::Number(1))
    );
    assert_eq!(
        modulo!([Exp::Atom(Atom::Number(4)), Exp::Atom(Atom::Number(2))]),
        Exp::Atom(Atom::Number(0))
    );
}
#[test]
fn eval_proc() {
    let s_exp = Exp::Atom(Atom::Symbol("s".to_string()));
    let n_exp = Exp::Atom(Atom::Number(7));
    let f_exp = Exp::Atom(Atom::Float(7.4));
    let l_exp = Exp::List(vec![Exp::Atom(Atom::Float(7.4))]);
    let operation = Exp::List(vec![
        Exp::Atom(Atom::Symbol("+".to_string())),
        Exp::Atom(Atom::Number(1)),
        Exp::Atom(Atom::Number(1)),
    ]);
    /*assert_eq!(
        Exp::Atom(Atom::Symbol("+".to_string())),
        s_exp.eval_proc().unwrap()
    );*/
    assert_eq!(Exp::Atom(Atom::Number(2)), operation.eval_proc().unwrap());
}
#[test]
fn as_atom() {
    let s_exp = Exp::Atom(Atom::Symbol("s".to_string()));
    let n_exp = Exp::Atom(Atom::Number(7));
    let f_exp = Exp::Atom(Atom::Float(7.4));
    let l_exp = Exp::List(vec![Exp::Atom(Atom::Float(7.4))]);
    assert_eq!((), s_exp.print());
    assert_eq!((), n_exp.print());
    assert_eq!((), f_exp.print());
    assert_eq!((), l_exp.print());
}
#[test]
fn eval() {
    let s_exp = Exp::Atom(Atom::Symbol("s".to_string()));
    let n_exp = Exp::Atom(Atom::Number(7));
    let f_exp = Exp::Atom(Atom::Float(7.4));
    let l_exp = Exp::List(vec![Exp::Atom(Atom::Symbol("s".to_string()))]);
    assert_eq!(
        Exp::Atom(Atom::Symbol("s".to_string())),
        s_exp.eval().unwrap()
    );
    /*assert_eq!(Exp::Atom(Atom::Number(7)), n_exp.eval().unwrap());
    assert_eq!(
        Exp::Atom(Atom::Float(7.4))
        f_exp.eval().unwrap()
    );*/
    assert_eq!(
        Exp::Atom(Atom::Symbol("s".to_string())),
        l_exp.eval().unwrap()
    );
}
#[test]
#[should_panic]
fn as_vec() {
    let s_exp = Exp::Atom(Atom::Symbol("s".to_string()));
    let n_exp = Exp::Atom(Atom::Number(7));
    let f_exp = Exp::Atom(Atom::Float(7.4));
    let l_exp = Exp::List(vec![Exp::Atom(Atom::Float(7.4))]);
    assert_eq!(
        vec![Exp::Atom(Atom::Symbol("s".to_string()))],
        s_exp.as_vec()
    );
    assert_eq!(vec![Exp::Atom(Atom::Number(7))], n_exp.as_vec());
    assert_eq!(vec![Exp::Atom(Atom::Float(7.4))], f_exp.as_vec());
    assert_eq!(
        vec![Exp::List(vec![Exp::Atom(Atom::Symbol("s".to_string()))])],
        l_exp.as_vec()
    );
}
#[test]
fn eval_print() {
    let s_exp = Exp::Atom(Atom::Symbol("s".to_string()));
    let n_exp = Exp::Atom(Atom::Number(7));
    let f_exp = Exp::Atom(Atom::Float(7.4));
    let l_exp = Exp::List(vec![Exp::Atom(Atom::Float(7.4))]);
    assert_eq!((), s_exp.print());
    assert_eq!((), n_exp.print());
    assert_eq!((), f_exp.print());
    assert_eq!((), l_exp.print());
}
#[test]
fn test_commands() {
    let mut commands = HashMap::new();
    commands.insert("command", vec!["ls", "-l", "-a"]);
    commands.insert("command", vec!["echo", "hello"]);
    for (k, v) in commands {
        let args: Vec<Exp> = v
            .iter()
            .map(|e| Exp::Atom(Atom::Symbol(e.to_string())))
            .collect();
        assert_eq!(
            Exp::Atom(Atom::Symbol(
                std::str::from_utf8(
                    &std::process::Command::new(v[0])
                        .args(&v[1..])
                        .output()
                        .unwrap()
                        .stdout
                )
                .unwrap()
                .to_string()
            )),
            ENV.lock()
                .unwrap()
                .eval(&Atom::Symbol("command".to_string()), args.as_slice(),)
                .unwrap()
        );
    }
}
#[test]
fn test_write() {
    let mut commands = HashMap::new();
    commands.insert("command", vec!["sh", "-c", "echo asdf > test.txt"]);
    for (k, v) in commands {
        let args: Vec<Exp> = v
            .iter()
            .map(|e| Exp::Atom(Atom::Symbol(e.to_string())))
            .collect();
        ENV.lock()
            .unwrap()
            .eval(&Atom::Symbol("command".to_string()), args.as_slice())
            .unwrap();
    }

    let cmd = &std::process::Command::new("file")
        .arg("test.txt")
        .output()
        .unwrap()
        .stdout;
    let output = std::str::from_utf8(cmd).unwrap();
    assert_ne!(
        "test.txt: cannot open `test.txt' (No such file or directory)\n",
        output
    );
}
