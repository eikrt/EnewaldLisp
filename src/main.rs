use enewaldlisp::parser;
use std::env::args;
fn main() {
    let arg = args().nth(1).unwrap();
    let parsed = parser::parse(arg.as_str()).unwrap();
    let output = parsed.eval().unwrap();
    output.print();
}
