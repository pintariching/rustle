use regex::{Captures, Regex};
use std::cmp::{max, min};

fn tabs_to_spaces(str: &str) -> String {
    let re = Regex::new(r"^\t+").unwrap();
    re.replace_all(str, |caps: &Captures| {
        let matched = &caps[0];
        matched.split("\t").collect::<Vec<&str>>().join("  ")
    })
    .to_string()
}

pub fn get_code_frame(source: String, line: isize, column: isize) -> String {
    let lines = source.split("\n").collect::<Vec<&str>>();
    let (frame_start, frame_end) = (max(0, line - 2), min(line + 3, lines.len() as isize));
    let frame_start = frame_start as usize;
    let frame_end = frame_end as usize;
    let digits = format!("{}", frame_end + 1).len();
    let joined = &lines[frame_start as usize..frame_end as usize]
        .into_iter()
        .enumerate()
        .map(|t| {
            let line_num = format!("{:digits$}", t.0 + frame_start + 1);

            if frame_start + t.0 == line as usize {
                let indicator = " ".repeat(digits + 2 + tabs_to_spaces(&t.1[0..column.try_into().unwrap()]).len()) + "^";
                return format!("{line_num}: {}\n{indicator}", tabs_to_spaces(*t.1));
            }

            return format!("{line_num}: {}", tabs_to_spaces(*t.1));
        })
        .collect::<Vec<String>>()
        .join("\n");

    joined.to_owned()
}