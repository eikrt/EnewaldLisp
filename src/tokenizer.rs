pub fn tokenize(input: &str) -> Vec<String> {
    let spaced = input.replace("(", " ( ").replace(")", " ) ");
    spaced.split_whitespace().map(String::from).collect()
}
