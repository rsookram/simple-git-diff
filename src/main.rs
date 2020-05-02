use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();

    stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok().map(transform))
        .for_each(|line| println!("{}", line));
}

fn transform(line: String) -> String {
    if is_index(&line) {
        String::new()
    } else if is_addition(&line) {
        line[6..].to_string()
    } else if is_removal(&line) {
        let mut without_minus = String::with_capacity(line.len() - 1);
        without_minus.push_str("\x1B[31m");
        without_minus.push_str(&line[6..]);

        without_minus
    } else if is_context(&line) {
        line[1..].to_string()
    } else {
        line
    }
}

fn is_index(line: &str) -> bool {
    line.starts_with("\x1B[1mindex ")
}

fn is_addition(line: &str) -> bool {
    line.starts_with("\x1B[32m+")
}

fn is_removal(line: &str) -> bool {
    line.starts_with("\x1B[31m-")
}

fn is_context(line: &str) -> bool {
    line.starts_with(' ')
}
