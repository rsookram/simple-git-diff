pub struct State {
    width: usize,
}

impl State {
    pub fn new() -> Self {
        let size = term_size::dimensions();
        let width = size.map_or(79, |(w, _)| w) as usize;

        State { width }
    }

    pub fn next(&self, mut line: String) -> Option<String> {
        match Line::type_of(&line) {
            Line::Diff => {
                let mut new_line = horizontal_rule(self.width);

                // Best effort to find old and new filename
                if line.matches(" b/").take(2).count() > 1 {
                    new_line.push_str(&line.replacen("diff --git a/", "", 1));
                    return Some(new_line);
                }

                let old_file_end = if let Some(i) = line.find(" b/") {
                    i
                } else {
                    new_line.push_str(&line.replacen("diff --git a/", "", 1));
                    return Some(new_line);
                };

                let mut result = String::new();
                result.push_str(&horizontal_rule(self.width));
                result.push('\n');

                let old_file_name = &line[17..old_file_end];
                result.push_str("\x1B[1m -- ");
                result.push_str(old_file_name);

                result.push('\n');

                let new_file_name = &line[old_file_end + 3..];
                result.push_str("\x1B[1m ++ ");
                result.push_str(new_file_name);

                Some(result)
            }
            Line::Index | Line::OldFilepath => None,
            Line::NewFilepath => Some(horizontal_rule(self.width)),
            Line::BinaryFilesDiffer => {
                line.push_str("\n");
                line.push_str(&horizontal_rule(self.width));
                Some(line)
            }
            Line::Addition => {
                let mut line = line[6..].to_string();
                // Mark the first character of an added newline
                if line.len() < 10 {
                    line.push_str("\x1B[32m\x1B[7m \x1B[m");
                }
                Some(line)
            }
            Line::Removal => {
                let mut line = line.replacen('-', "", 1);

                // Mark the first character of a removed newline
                if line.len() < 10 {
                    line.push_str("\x1B[31m\x1B[7m \x1B[m");
                }

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

fn horizontal_rule(length: usize) -> String {
    let em_dash = "\u{2500}";

    em_dash.repeat(length)
}
