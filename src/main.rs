use enewaldlisp::parser;
use std::env::args;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let arg = args().nth(1).unwrap();
    if let Ok(lines) = read_lines(&arg) {
        for line in lines {
            if let Ok(ip) = line {
                let parsed = parser::parse(&ip).unwrap();
                let output = parsed.eval().unwrap();
                output.print();
            }
        }
    } else {
        let parsed = parser::parse(&arg).unwrap();
        let output = parsed.eval().unwrap();
        output.print();
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
