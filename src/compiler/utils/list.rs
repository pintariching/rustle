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
