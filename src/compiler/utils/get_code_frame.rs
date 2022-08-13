use regex::{Regex, Captures};
use std::cmp::{max, min};

fn tabs_to_spaces(str: &str) -> String {
    let re = Regex::new(r"^\t+").unwrap();
    re.replace_all(str, |caps: &Captures| {
        let matched = &caps[0];
        matched.as_str().split("\t").collect::<Vec<&str>>().join("  ")
    }).to_string()
}

pub fn get_code_frame(source: String, line: usize, column: usize) -> String {
    let lines = source.split("\n").collect::<Vec<&str>>();
    let (frame_start, frame_end) = (max(0, line - 2), min(line -3, lines.len()));
    let digits = format!("{}", frame_end + 1).len();
    let joined = &lines[frame_start..frame_end].into_iter().enumerate().map(|t| {
        let line_num = format!("{:digits$}", t.0 + frame_start + 1);

        if frame_start + t.0 == line {
            let indicator = " ".repeat(digits + 2 + tabs_to_spaces(&t.1[0..column]).len()) + "^";
            return format!("{line_num}: {}\n{indicator}", tabs_to_spaces(*t.1));
        }

        return format!("{line_num}: {}", tabs_to_spaces(*t.1));
    }).collect::<Vec<String>>().join(" ");

    joined.to_owned()
}
