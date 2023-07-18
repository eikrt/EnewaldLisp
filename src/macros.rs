#[macro_export]
macro_rules! plus {
    ($slice:expr) => {{
        let numbers: Vec<i64> = $slice
            .iter()
            .filter_map(|exp| {
                if let lex::Exp::Atom(lex::Atom::Number(num)) = exp {
                    Some(*num)
                } else {
                    None
                }
            })
            .collect();

        lex::Exp::Atom(lex::Atom::Number(numbers.iter().sum()))
    }};
}
#[macro_export]
macro_rules! minus {
    ($slice:expr) => {{
        let first = &$slice[0];
        let val: i64 = $slice.iter().skip(1).fold(
            match first {
                lex::Exp::Atom(a) => match a {
                    lex::Atom::Number(n) => *n,
                    _ => todo!(),
                },
                _ => todo!(),
            },
            |acc, num| {
                acc - match num {
                    lex::Exp::Atom(a) => match a {
                        lex::Atom::Number(n) => n,
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            },
        );
        lex::Exp::Atom(lex::Atom::Number(val))
    }};
}
#[macro_export]
macro_rules! multi {
    ($slice:expr) => {{
        let numbers: Vec<i64> = $slice
            .iter()
            .filter_map(|exp| {
                if let lex::Exp::Atom(lex::Atom::Number(num)) = exp {
                    Some(*num)
                } else {
                    None
                }
            })
            .collect();

        lex::Exp::Atom(lex::Atom::Number(numbers.iter().product()))
    }};
}
#[macro_export]
macro_rules! div {
    ($slice:expr) => {{
        let first = &$slice[0];
        let val: i64 = $slice.iter().skip(1).fold(
            match first {
                lex::Exp::Atom(a) => match a {
                    lex::Atom::Number(n) => *n,
                    _ => todo!(),
                },
                _ => todo!(),
            },
            |acc, num| {
                acc / match num {
                    lex::Exp::Atom(a) => match a {
                        lex::Atom::Number(n) => n,
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            },
        );
        lex::Exp::Atom(lex::Atom::Number(val))
    }};
}
#[macro_export]
macro_rules! modulo {
    ($slice:expr) => {{
        let first = &$slice[0];
        let val: i64 = $slice.iter().skip(1).fold(
            match first {
                lex::Exp::Atom(a) => match a {
                    lex::Atom::Number(n) => *n,
                    _ => todo!(),
                },
                _ => todo!(),
            },
            |acc, num| {
                acc % match num {
                    lex::Exp::Atom(a) => match a {
                        lex::Atom::Number(n) => n,
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            },
        );
        lex::Exp::Atom(lex::Atom::Number(val))
    }};
}
