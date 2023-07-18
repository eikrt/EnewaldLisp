use crate::lex::Atom;
use crate::lex::Exp;
use crate::tokenizer;

pub fn parse(program: &str) -> Result<Exp, ()> {
    let mut tokens = tokenizer::tokenize(program);
    let rtokens = read_from_tokens(&mut tokens)?;
    Ok(Exp::List(rtokens))
}

pub fn read_from_tokens(tokens: &mut Vec<String>) -> Result<Vec<Exp>, ()> {
    let binding = tokens.remove(0);
    let token = binding.as_str();
    match token {
        "(" => {
            let mut l: Vec<Exp> = Vec::new();
            while tokens[0] != ")" {
                l.extend(read_from_tokens(tokens)?);
            }
            tokens.remove(0);
            Ok(vec![Exp::List(l)])
        }
        ")" => Err(()),
        _ => Ok(vec![Exp::Atom(atom(token))]),
    }
}

pub fn atom(token: &str) -> Atom {
    if let Ok(number) = token.parse::<i64>() {
        Atom::Number(number)
    } else if let Ok(float) = token.parse::<f64>() {
        Atom::Float(float)
    } else {
        Atom::Symbol(token.to_string())
    }
}
