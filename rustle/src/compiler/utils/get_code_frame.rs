use regex::{Captures, Regex};
use std::cmp::{max, min};

fn tabs_to_spaces(str: &str) -> String {
    let re = Regex::new(r"^\t+").unwrap();
    re.replace_all(str, |caps: &Captures| {
        let matched = &caps[0];
        matched.split('\t').collect::<Vec<&str>>().join("  ")
    })
    .to_string()
}

pub fn get_code_frame(source: String, line: usize, column: usize) -> String {
    let lines = source.split('\n').collect::<Vec<&str>>();
    let line_start = if line <= 2 { 0 } else { line - 2 };
    let line_end = line + 3;
    let (frame_start, frame_end) = (max(0, line_start), min(line_end, lines.len()));
    let digits = format!("{}", frame_end + 1).len();
    let joined = &lines[frame_start..frame_end]
        .iter()
        .enumerate()
        .map(|t| {
            let line_num = format!("{:digits$}", t.0 + frame_start + 1);

            if frame_start + t.0 == line {
                let indicator = " "
                    .repeat(digits + 2 + tabs_to_spaces(&t.1[0..column.try_into().unwrap()]).len())
                    + "^";
                return format!("{line_num}: {}\n{indicator}", tabs_to_spaces(*t.1));
            }

            return format!("{line_num}: {}", tabs_to_spaces(*t.1));
        })
        .collect::<Vec<String>>()
        .join("\n");

    joined.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_code_frame_with_line_0() {
        let actual = get_code_frame("we has a long error".to_string(), 0, 3);
        let expected = "1: we has a long error\n      ^";
        assert_eq!(actual, expected)
    }

    #[test]
    fn get_code_frame_with_line_1() {
        let actual = get_code_frame(
            "we has a long error\nwe has a long error\nwe has a long error".to_string(),
            1,
            3,
        );
        let expected =
            "1: we has a long error\n2: we has a long error\n      ^\n3: we has a long error";
        assert_eq!(actual, expected)
    }
}
