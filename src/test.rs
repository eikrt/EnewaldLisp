use crate::environment;
use crate::lex;
use crate::macros;
use crate::parser;
use crate::test;
use crate::tokenizer;
use crate::{div, minus, modulo, multi, plus};
#[cfg(test)]
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
        lex::Exp::List(vec![lex::Exp::List(vec![
            lex::Exp::Atom(lex::Atom::Symbol("+".to_string())),
            lex::Exp::Atom(lex::Atom::Number(1)),
            lex::Exp::Atom(lex::Atom::Number(1)),
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
        vec![lex::Exp::List(vec![
            lex::Exp::Atom(lex::Atom::Symbol("+".to_string())),
            lex::Exp::Atom(lex::Atom::Number(1)),
            lex::Exp::Atom(lex::Atom::Number(1))
        ])]
    )
}
#[test]
fn atom() {
    assert_eq!(lex::Atom::Symbol("x".to_string()), parser::atom("x"));
    assert_eq!(lex::Atom::Number(7), parser::atom("7"));
    assert_eq!(lex::Atom::Float(7.1), parser::atom("7.1"));
}
#[test]
fn plus() {
    assert_eq!(
        plus!([
            lex::Exp::Atom(lex::Atom::Number(1)),
            lex::Exp::Atom(lex::Atom::Number(1))
        ]),
        lex::Exp::Atom(lex::Atom::Number(2))
    );
}
#[test]
fn minus() {
    assert_eq!(
        plus!([
            lex::Exp::Atom(lex::Atom::Number(1)),
            lex::Exp::Atom(lex::Atom::Number(1))
        ]),
        lex::Exp::Atom(lex::Atom::Number(2))
    );
}
#[test]
fn multi() {
    assert_eq!(
        multi!([
            lex::Exp::Atom(lex::Atom::Number(2)),
            lex::Exp::Atom(lex::Atom::Number(2))
        ]),
        lex::Exp::Atom(lex::Atom::Number(4))
    );
}
#[test]
fn div() {
    assert_eq!(
        div!([
            lex::Exp::Atom(lex::Atom::Number(4)),
            lex::Exp::Atom(lex::Atom::Number(2))
        ]),
        lex::Exp::Atom(lex::Atom::Number(2))
    );
}
#[test]
fn modulo() {
    assert_eq!(
        modulo!([
            lex::Exp::Atom(lex::Atom::Number(5)),
            lex::Exp::Atom(lex::Atom::Number(2))
        ]),
        lex::Exp::Atom(lex::Atom::Number(1))
    );
    assert_eq!(
        modulo!([
            lex::Exp::Atom(lex::Atom::Number(4)),
            lex::Exp::Atom(lex::Atom::Number(2))
        ]),
        lex::Exp::Atom(lex::Atom::Number(0))
    );
}
#[test]
fn eval_proc() {
    let s_exp = lex::Exp::Atom(lex::Atom::Symbol("s".to_string()));
    let n_exp = lex::Exp::Atom(lex::Atom::Number(7));
    let f_exp = lex::Exp::Atom(lex::Atom::Float(7.4));
    let l_exp = lex::Exp::List(vec![lex::Exp::Atom(lex::Atom::Float(7.4))]);
    let operation = lex::Exp::List(vec![
        lex::Exp::Atom(lex::Atom::Symbol("+".to_string())),
        lex::Exp::Atom(lex::Atom::Number(1)),
        lex::Exp::Atom(lex::Atom::Number(1)),
    ]);
    /*assert_eq!(
        lex::Exp::Atom(lex::Atom::Symbol("+".to_string())),
        s_exp.eval_proc().unwrap()
    );*/
    assert_eq!(
        lex::Exp::Atom(lex::Atom::Number(2)),
        operation.eval_proc().unwrap()
    );
}
#[test]
fn as_atom() {
    let s_exp = lex::Exp::Atom(lex::Atom::Symbol("s".to_string()));
    let n_exp = lex::Exp::Atom(lex::Atom::Number(7));
    let f_exp = lex::Exp::Atom(lex::Atom::Float(7.4));
    let l_exp = lex::Exp::List(vec![lex::Exp::Atom(lex::Atom::Float(7.4))]);
    assert_eq!((), s_exp.print());
    assert_eq!((), n_exp.print());
    assert_eq!((), f_exp.print());
    assert_eq!((), l_exp.print());
}
#[test]
#[test]
fn eval() {
    let s_exp = lex::Exp::Atom(lex::Atom::Symbol("s".to_string()));
    let n_exp = lex::Exp::Atom(lex::Atom::Number(7));
    let f_exp = lex::Exp::Atom(lex::Atom::Float(7.4));
    let l_exp = lex::Exp::List(vec![lex::Exp::Atom(lex::Atom::Symbol("s".to_string()))]);
    assert_eq!(
        lex::Exp::Atom(lex::Atom::Symbol("s".to_string())),
        s_exp.eval().unwrap()
    );
    /*assert_eq!(lex::Exp::Atom(lex::Atom::Number(7)), n_exp.eval().unwrap());
    assert_eq!(
        lex::Exp::Atom(lex::Atom::Float(7.4))
        f_exp.eval().unwrap()
    );*/
    assert_eq!(
        lex::Exp::Atom(lex::Atom::Symbol("s".to_string())),
        l_exp.eval().unwrap()
    );
}
#[test]
#[should_panic]
fn as_vec() {
    let s_exp = lex::Exp::Atom(lex::Atom::Symbol("s".to_string()));
    let n_exp = lex::Exp::Atom(lex::Atom::Number(7));
    let f_exp = lex::Exp::Atom(lex::Atom::Float(7.4));
    let l_exp = lex::Exp::List(vec![lex::Exp::Atom(lex::Atom::Float(7.4))]);
    assert_eq!(
        vec![lex::Exp::Atom(lex::Atom::Symbol("s".to_string()))],
        s_exp.as_vec()
    );
    assert_eq!(vec![lex::Exp::Atom(lex::Atom::Number(7))], n_exp.as_vec());
    assert_eq!(vec![lex::Exp::Atom(lex::Atom::Float(7.4))], f_exp.as_vec());
    assert_eq!(
        vec![lex::Exp::List(vec![lex::Exp::Atom(lex::Atom::Symbol(
            "s".to_string()
        ))])],
        l_exp.as_vec()
    );
}
#[test]
fn eval_print() {
    let s_exp = lex::Exp::Atom(lex::Atom::Symbol("s".to_string()));
    let n_exp = lex::Exp::Atom(lex::Atom::Number(7));
    let f_exp = lex::Exp::Atom(lex::Atom::Float(7.4));
    let l_exp = lex::Exp::List(vec![lex::Exp::Atom(lex::Atom::Float(7.4))]);
    assert_eq!((), s_exp.print());
    assert_eq!((), n_exp.print());
    assert_eq!((), f_exp.print());
    assert_eq!((), l_exp.print());
}
