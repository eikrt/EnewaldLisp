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

        numbers.iter().sum()
    }};
}
