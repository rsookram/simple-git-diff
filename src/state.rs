pub struct State {}

impl State {
    pub fn next(mut line: String) -> Option<String> {
        if is_diff(&line) {
            None
        } else if is_index(&line) {
            Some(horizontal_rule())
        } else if is_old_filepath(&line) {
            Some(line.replacen('-', " ", 1))
        } else if is_new_filepath(&line) {
            let mut line = line.replacen('+', " ", 1);

            line.push_str("\n");
            line.push_str(&horizontal_rule());

            Some(line)
        } else if is_binary_files_differ(&line) {
            line.push_str("\n");
            line.push_str(&horizontal_rule());
            Some(line)
        } else if is_addition(&line) {
            Some(line[6..].to_string())
        } else if is_removal(&line) {
            let line = line.replacen('-', "", 1);

            Some(line)
        } else if is_context(&line) {
            Some(line[1..].to_string())
        } else if is_no_newline_at_eof(&line) {
            line.push_str("\n");
            Some(line)
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

fn is_binary_files_differ(line: &str) -> bool {
    line.starts_with("Binary files ") && line.ends_with(" differ")
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

fn is_no_newline_at_eof(line: &str) -> bool {
    line.starts_with("\\ No newline at end of file")
}

fn horizontal_rule() -> String {
    let em_dash = "\u{2500}";
    let length = 79;

    em_dash.repeat(length)
}
