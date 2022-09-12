#[macro_export]
macro_rules! list {
    ($a:expr, $b:expr) => {{
        let len = $a.len();
        if len == 1 {
            $a.get(0).unwrap().to_string()
        } else {
            format!("{} {} {}", $a[..len - 1].join(", "), $b, $a.last().unwrap())
        }
    }};
    ($a:expr) => {{
        let len = $a.len();
        if len == 1 {
            $a.get(0).unwrap().to_string()
        } else {
            format!("{} or {}", $a[..len - 1].join(", "), $a.last().unwrap())
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_list_macro() {
        let list = list!(vec!["one", "two"]);
        assert_eq!(list, String::from("one or two"));

        let list = list!(vec!["one", "two", "three", "four"]);
        assert_eq!(list, String::from("one, two, three or four"));

        let list = list!(vec!["one", "two"], "and");
        assert_eq!(list, String::from("one and two"));

        let list = list!(vec!["one", "two", "three", "four"], "and");
        assert_eq!(list, String::from("one, two, three and four"));
    }
}
