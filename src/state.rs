pub struct State {}

impl State {
    pub fn next(line: String) -> Option<String> {
        if is_diff(&line) {
            None
        } else if is_index(&line) {
            Some(horizontal_rule())
        } else if is_old_filepath(&line) {
            Some(line.replacen('-', " ", 1))
        } else if is_new_filepath(&line) {
            Some(line.replacen('+', " ", 1))
        } else if is_addition(&line) {
            Some(line[6..].to_string())
        } else if is_removal(&line) {
            let mut without_minus = String::with_capacity(line.len() - 1);
            without_minus.push_str("\x1B[31m");
            without_minus.push_str(&line[6..]);

            Some(without_minus)
        } else if is_context(&line) {
            Some(line[1..].to_string())
        } else {
            Some(line)
        }
    }
}

fn is_diff(line: &str) -> bool {
    line.starts_with("\x1B[1mdiff ")
}

fn is_index(line: &str) -> bool {
    line.starts_with("\x1B[1mindex ")
}

fn is_old_filepath(line: &str) -> bool {
    line.starts_with("\x1B[1m--- ")
}

fn is_new_filepath(line: &str) -> bool {
    line.starts_with("\x1B[1m+++ ")
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

fn horizontal_rule() -> String {
    let length = 79;
    // \u{2500} is an em dash
    "\u{2500}".repeat(length)
}
