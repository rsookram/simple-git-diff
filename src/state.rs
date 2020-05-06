pub struct State {}

impl State {
    pub fn next(mut line: String) -> Option<String> {
        match Line::type_of(&line) {
            Line::Diff => {
                line.clear();
                Some(line)
            }
            Line::Index => Some(horizontal_rule()),
            Line::OldFilepath => Some(line.replacen('-', " ", 1)),
            Line::NewFilepath => {
                let mut line = line.replacen('+', " ", 1);

                line.push_str("\n");
                line.push_str(&horizontal_rule());

                Some(line)
            }
            Line::BinaryFilesDiffer => {
                line.push_str("\n");
                line.push_str(&horizontal_rule());
                Some(line)
            }
            Line::Addition => Some(line[6..].to_string()),
            Line::Removal => {
                let line = line.replacen('-', "", 1);

                Some(line)
            }
            Line::Context => Some(line[1..].to_string()),
            Line::Other => Some(line),
        }
    }
}

enum Line {
    Diff,
    Index,
    OldFilepath,
    NewFilepath,
    BinaryFilesDiffer,
    Addition,
    Removal,
    Context,
    Other,
}

impl Line {
    fn type_of(line: &str) -> Self {
        if line.starts_with("\x1B[1mdiff ") {
            Line::Diff
        } else if line.starts_with("\x1B[1mindex ") {
            Line::Index
        } else if line.starts_with("\x1B[1m--- ") {
            Line::OldFilepath
        } else if line.starts_with("\x1B[1m+++ ") {
            Line::NewFilepath
        } else if line.starts_with("Binary files ") && line.ends_with(" differ") {
            Line::BinaryFilesDiffer
        } else if line.starts_with("\x1B[32m+") {
            Line::Addition
        } else if line.starts_with("\x1B[31m-") {
            Line::Removal
        } else if line.starts_with(' ') {
            Line::Context
        } else {
            Line::Other
        }
    }
}

fn horizontal_rule() -> String {
    let em_dash = "\u{2500}";
    let length = 79;

    em_dash.repeat(length)
}
